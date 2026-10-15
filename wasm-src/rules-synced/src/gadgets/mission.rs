pub(crate) mod behavior;

use spring_addons::AddonContext;
use spring_addons::rules::Gadget;

use crate::gadgets::mission::behavior::{self as mission, Mission};
use crate::state::{Global, PerLevel};

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
