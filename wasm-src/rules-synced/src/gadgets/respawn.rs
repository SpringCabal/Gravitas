use spring_addons::AddonContext;
use spring_addons::rules::{Gadget, UnitDestroyedEvent};

use crate::respawn::{self, Respawns};
use crate::state::{Global, PerLevel};
use crate::{effects, units};

/// Respawnable units come back where they started when they die or get stuck.
#[derive(Default)]
pub struct RespawnGadget {
    respawns: PerLevel<Respawns>,
}

impl Gadget<Global> for RespawnGadget {
    fn name(&self) -> &'static str {
        "Respawn"
    }

    fn unit_created(
        &self,
        ctx: &AddonContext<'_, Global>,
        unit_id: i32,
        unit_def_id: i32,
        team: i32,
        _builder: i32,
    ) {
        if effects::is_respawnable(unit_id) {
            self.respawns
                .get(ctx.global())
                .adopt(unit_id, unit_def_id, team, units::game_frame());
        }
    }

    fn unit_destroyed(&self, ctx: &AddonContext<'_, Global>, event: &UnitDestroyedEvent) {
        let global = ctx.global();
        let rebirth = self.respawns.get(global).destroyed(event.unit_id);
        if let Some(rebirth) = rebirth
            && !global.restart.wiping()
        {
            // Not from inside UnitDestroyed: the new unit's UnitCreated would
            // run while the engine is still removing the old one.
            ctx.delay(move |_| {
                respawn::rebirth(&rebirth);
            });
        }
    }

    fn game_frame(&self, ctx: &AddonContext<'_, Global>, frame: i32) {
        let rebirths = {
            let mut respawns = self.respawns.get(ctx.global());
            if frame == respawn::SCAN_FRAME {
                respawns.scan(frame);
            }
            respawns.update(frame)
        };
        for rebirth in &rebirths {
            respawn::rebirth(rebirth);
        }
    }
}
