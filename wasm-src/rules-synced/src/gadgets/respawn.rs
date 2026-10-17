pub(crate) mod behavior;

use alloc::rc::Rc;

use spring_addons::AddonContext;
use spring_addons::rules::{Gadget, UnitDestroyedEvent};

use crate::effects;
use crate::gadgets::respawn::behavior::{self as respawn, Respawns};
use crate::state::{Global, PerLevel};

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
        let spring = ctx.spring();
        if effects::is_respawnable(spring, unit_id) {
            self.respawns.get(ctx.global()).adopt(
                spring,
                unit_id,
                unit_def_id,
                team,
                spring.game().current_frame().unwrap_or(0),
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
        let spring = ctx.spring();
        let mut respawns = self.respawns.get(ctx.global());
        if frame == respawn::SCAN_FRAME {
            respawns.scan(spring, frame);
        }
        for rebirth in respawns.update(spring, frame) {
            let respawns = Rc::clone(&self.respawns);
            let (unit_def_id, team_id) = (rebirth.unit_def_id, rebirth.team_id);
            respawn::rebirth(spring, &rebirth, move |spring, unit_id| {
                crate::with_global(|global| {
                    respawns
                        .get(global)
                        .adopt(spring, unit_id, unit_def_id, team_id, frame);
                });
            });
        }
    }
}
