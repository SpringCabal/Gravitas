pub(crate) mod restart;

pub(crate) mod setup;

pub(crate) mod data;

use spring::commands;
use spring_addons::AddonContext;
use spring_addons::rules::{Gadget, UnitDestroyedEvent};

use crate::diagnostics;
use crate::gadgets::level::data as level;
use crate::gadgets::mission::behavior as mission;
use crate::state::{Global, Phase};

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
            Phase::Spawn => commands::run(|| crate::with_global(setup::spawn)),
            Phase::Link => setup::link(global),
            Phase::Running => {}
        }
    }

    fn unit_destroyed(&self, ctx: &AddonContext<'_, Global>, event: &UnitDestroyedEvent) {
        let global = ctx.global();
        if global.restart.wiping() {
            return;
        }
        let boss = global.level.access().unit_of(level::BOSS_UNIT);
        if boss == Some(event.unit_id) {
            mission::win(global, &[PLAYER_ALLY_TEAM]);
        }
    }
}

const PLAYER_ALLY_TEAM: i32 = 0;
