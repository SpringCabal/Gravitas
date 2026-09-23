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
            emit_spark(shock.unit_id, shock.intensity, frame);
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

fn emit_spark(unit_id: i32, intensity: f32, frame: i32) {
    let Some(mut position) = units::position(unit_id) else {
        return;
    };
    // Like the Lua gadget, start every spark at an independent point inside a
    // radius/2 cube around the unit.
    let radius = spring::units_info::get_unit_radius(unit_id).unwrap_or(0.0);
    let spread = radius * 0.5;
    let seed = (unit_id as u64)
        .wrapping_mul(2_862_933_555_777_941_757)
        .wrapping_add(frame as u64);
    position.x += spread * signed_random(seed);
    position.y += spread * signed_random(seed.wrapping_add(1));
    position.z += spread * signed_random(seed.wrapping_add(2));
    let velocity = spring::units_info::get_unit_velocity(unit_id).unwrap_or_default();
    let _ = spring::send_to_unsynced(&format!(
        "GRAVITAS_FX|spark|{}|{}|{}|{intensity}|{}|{}",
        position.x, position.y, position.z, velocity.x, velocity.z
    ));
}

fn signed_random(seed: u64) -> f32 {
    let mixed = seed
        .wrapping_mul(6_364_136_223_846_793_005)
        .wrapping_add(1_442_695_040_888_963_407);
    let unit = ((mixed >> 40) as u32) as f32 / 16_777_216.0;
    unit * 2.0 - 1.0
}
