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
        self.respawns
            .get(global)
            .destroyed(event.unit_id, global.restart.wiping());
    }

    fn game_frame(&self, ctx: &AddonContext<'_, Global>, frame: i32) {
        let global = ctx.global();
        let due = {
            let mut respawns = self.respawns.get(global);
            if frame == respawn::SCAN_FRAME {
                respawns.scan(frame);
            }
            respawns.update(frame)
        };
        // Destroying and creating units raises UnitDestroyed and UnitCreated,
        // which borrow the respawns: none is held here.
        for rebirth in &due {
            if let Some(unit_id) = respawn::rebirth(rebirth) {
                self.respawns.get(global).adopt(
                    unit_id,
                    rebirth.unit_def_id,
                    rebirth.team_id,
                    frame,
                );
            }
        }
    }
}
