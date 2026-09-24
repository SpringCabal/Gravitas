use spring::ResultExt;
use spring_addons::AddonContext;
use spring_addons::rules::{Gadget, UnitDestroyedEvent};

use gravitas_shared::EffectKind;

use crate::effects::send_effect;
use crate::electrafi::Electrafi;
use crate::state::{Global, PerLevel};
use crate::units;

/// Electric fields shock the player and stun robots.
#[derive(Default)]
pub struct ElectrafiGadget {
    electrafi: PerLevel<Electrafi>,
}

impl Gadget<Global> for ElectrafiGadget {
    fn name(&self) -> &'static str {
        "Electrafi"
    }

    fn unit_created(
        &self,
        ctx: &AddonContext<'_, Global>,
        unit_id: i32,
        unit_def_id: i32,
        _team: i32,
        _builder: i32,
    ) {
        self.electrafi
            .get(ctx.global())
            .created(unit_id, unit_def_id);
    }

    fn unit_destroyed(&self, ctx: &AddonContext<'_, Global>, event: &UnitDestroyedEvent) {
        self.electrafi.get(ctx.global()).destroyed(event.unit_id);
    }

    fn game_frame(&self, ctx: &AddonContext<'_, Global>, frame: i32) {
        let (shocks, arcs) = {
            let mut electrafi = self.electrafi.get(ctx.global());
            (electrafi.update(frame), electrafi.arcs(frame))
        };
        for shock in shocks {
            // An earlier shock this frame may have killed it.
            if !units::valid(shock.unit_id) {
                continue;
            }
            if let Some(damage) = shock.damage {
                let paralyze_time = if shock.paralyze { PARALYZING } else { 0.0 };
                spring::add_unit_damage(
                    shock.unit_id,
                    damage,
                    paralyze_time,
                    -1,
                    shock.attacker_id,
                    spring::types::Float3::default(),
                )
                .log_err_once("electric field damage");
            }
            emit_spark(shock.unit_id, shock.intensity);
            crate::tests::note_spark(ctx.global(), shock.unit_id, shock.damage.is_some());
        }
        for arc in arcs {
            send_effect(arc.position, EffectKind::Arc { radius: arc.radius });
        }
    }
}

const PARALYZING: f32 = 1.0;

/// The ui module scatters the sparks over a cube of half the unit's radius.
fn emit_spark(unit_id: i32, intensity: f32) {
    let Some(position) = units::position(unit_id) else {
        return;
    };
    let spread = spring::units_info::get_unit_radius(unit_id).unwrap_or(0.0) * 0.5;
    let velocity = spring::units_info::get_unit_velocity(unit_id).unwrap_or_default();
    send_effect(
        position,
        EffectKind::Spark {
            intensity,
            velocity: [velocity.x, velocity.z],
            spread,
        },
    );
}
