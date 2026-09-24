use spring::ResultExt;
use spring_addons::AddonContext;
use spring_addons::rules::{Gadget, UnitDestroyedEvent};

use gravitas_shared::EffectKind;

use crate::effects::send_effect;
use crate::fire::Fires;
use crate::state::{Global, PerLevel};
use crate::units;

/// Fire burns what stands in its flames, and keeps burning it for a while.
#[derive(Default)]
pub struct FireGadget {
    fires: PerLevel<Fires>,
}

impl Gadget<Global> for FireGadget {
    fn name(&self) -> &'static str {
        "Fire"
    }

    fn unit_created(
        &self,
        ctx: &AddonContext<'_, Global>,
        unit_id: i32,
        unit_def_id: i32,
        _team: i32,
        _builder: i32,
    ) {
        self.fires.get(ctx.global()).created(unit_id, unit_def_id);
    }

    fn unit_destroyed(&self, ctx: &AddonContext<'_, Global>, event: &UnitDestroyedEvent) {
        self.fires.get(ctx.global()).destroyed(event.unit_id);
    }

    fn game_frame(&self, ctx: &AddonContext<'_, Global>, frame: i32) {
        let (burns, flames) = {
            let mut fires = self.fires.get(ctx.global());
            (fires.update(frame), fires.flames(frame))
        };
        for burn in burns {
            // An earlier burn this frame may have killed it.
            if !units::valid(burn.unit_id) {
                continue;
            }
            spring::add_unit_damage(
                burn.unit_id,
                burn.damage,
                0.0,
                -1,
                burn.attacker_id,
                spring::types::Float3::default(),
            )
            .log_err_once("fire damage");
            emit_burn(burn.unit_id, burn.size_mult);
        }
        for flame in flames {
            send_effect(
                flame.position,
                EffectKind::Flame {
                    radius: flame.radius,
                },
            );
        }
    }
}

const DEFAULT_BURN_RADIUS: f32 = 20.0;

fn emit_burn(unit_id: i32, size_mult: f32) {
    let Some(position) = units::position(unit_id) else {
        return;
    };
    let radius = spring::units_info::get_unit_radius(unit_id).unwrap_or(DEFAULT_BURN_RADIUS);
    let velocity = spring::units_info::get_unit_velocity(unit_id).unwrap_or_default();
    send_effect(
        position,
        EffectKind::Burn {
            radius,
            strength: size_mult,
            velocity: [velocity.x, velocity.z],
        },
    );
}
