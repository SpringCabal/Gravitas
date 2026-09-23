use spring_addons::AddonContext;
use spring_addons::rules::Gadget;

use crate::mission::{self, Mission};
use crate::state::{Global, PerLevel};

/// Sends the level's robots into their assaults as the player enters trigger
/// areas.
#[derive(Default)]
pub struct MissionGadget {
    mission: PerLevel<Mission>,
}

impl Gadget<Global> for MissionGadget {
    fn name(&self) -> &'static str {
        "Mission"
    }

    fn game_frame(&self, ctx: &AddonContext<'_, Global>, frame: i32) {
        let global = ctx.global();
        let player_unit = global.player_unit.get();
        let assaults = self.mission.get(global).update(player_unit, frame);
        for assault in &assaults {
            mission::issue_assault(assault);
        }
    }
}
