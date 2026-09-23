use spring_addons::AddonContext;
use spring_addons::rules::{Gadget, UnitDestroyedEvent};

use crate::state::{Global, Phase};
use crate::{diagnostics, level, mission, restart, setup};

/// Spawns the level, links it on the next frame, restarts it on request, and
/// ends the game when the player or the boss dies.
pub struct LevelGadget;

impl Gadget<Global> for LevelGadget {
    fn name(&self) -> &'static str {
        "Level"
    }

    fn game_frame(&self, ctx: &AddonContext<'_, Global>, _frame: i32) {
        let global = ctx.global();
        if global.restart.take_request() {
            restart::perform(global);
            diagnostics::info("restarting the level");
            return;
        }
        let phase = global.level.access().phase;
        match phase {
            Phase::Spawn => setup::spawn(global),
            Phase::Link => setup::link(global),
            Phase::Running => {}
        }
    }

    fn unit_destroyed(&self, ctx: &AddonContext<'_, Global>, event: &UnitDestroyedEvent) {
        let global = ctx.global();
        if global.restart.wiping() {
            return;
        }
        if crate::defs::get().gravit == Some(event.unit_def_id) {
            mission::win(global, &[ENEMY_ALLY_TEAM]);
            return;
        }
        let boss = global.level.access().unit_of(level::BOSS_UNIT);
        if boss == Some(event.unit_id) {
            mission::win(global, &[PLAYER_ALLY_TEAM]);
        }
    }
}

const PLAYER_ALLY_TEAM: i32 = 0;
const ENEMY_ALLY_TEAM: i32 = 1;
