use alloc::rc::Rc;

use spring_addons::AddonContext;
use spring_addons::rules::{Gadget, UnitDestroyedEvent};

use crate::effects;
use crate::respawn::{self, Respawns};
use crate::state::{Global, PerLevel};

/// Respawnable units come back where they started when they die or get stuck.
#[derive(Default)]
pub struct RespawnGadget {
    respawns: Rc<PerLevel<Respawns>>,
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
            self.respawns.get(ctx.global()).adopt(
                unit_id,
                unit_def_id,
                team,
                spring::game::current_frame(),
            );
        }
    }

    fn unit_destroyed(&self, ctx: &AddonContext<'_, Global>, event: &UnitDestroyedEvent) {
        let global = ctx.global();
        self.respawns
            .get(global)
            .destroyed(event.unit_id, global.restart.wiping());
    }

    fn game_frame(&self, ctx: &AddonContext<'_, Global>, frame: i32) {
        let mut respawns = self.respawns.get(ctx.global());
        if frame == respawn::SCAN_FRAME {
            respawns.scan(frame);
        }
        for rebirth in respawns.update(frame) {
            let respawns = Rc::clone(&self.respawns);
            let (unit_def_id, team_id) = (rebirth.unit_def_id, rebirth.team_id);
            respawn::rebirth(&rebirth, move |unit_id| {
                crate::with_global(|global| {
                    respawns
                        .get(global)
                        .adopt(unit_id, unit_def_id, team_id, frame);
                });
            });
        }
    }
}
