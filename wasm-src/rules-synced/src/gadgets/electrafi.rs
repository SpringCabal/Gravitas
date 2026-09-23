use alloc::format;

use spring_addons::AddonContext;
use spring_addons::rules::{Gadget, UnitDestroyedEvent};

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
            let paralyze_time = if shock.paralyze { PARALYZING } else { 0.0 };
            let _ = spring::add_unit_damage(
                shock.unit_id,
                shock.damage,
                paralyze_time,
                -1,
                shock.attacker_id,
                spring::types::Float3::default(),
            );
            emit_spark(shock.unit_id, shock.intensity);
        }
        for arc in arcs {
            let _ = spring::send_to_unsynced(&format!(
                "GRAVITAS_FX|arc|{}|{}|{}|{}",
                arc.position.x, arc.position.y, arc.position.z, arc.radius
            ));
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
    let _ = spring::send_to_unsynced(&format!(
        "GRAVITAS_FX|spark|{}|{}|{}|{intensity}|{}|{}|{spread}",
        position.x, position.y, position.z, velocity.x, velocity.z
    ));
}
