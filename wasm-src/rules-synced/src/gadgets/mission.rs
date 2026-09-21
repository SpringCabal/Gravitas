use spring_addons::rules::Gadget;

use crate::gadgets::setup::frame_consumed;
use crate::orchestration;
use crate::state::Global;

#[derive(Default)]
pub struct MissionGadget;

impl MissionGadget {
    pub fn new() -> Self {
        Self
    }
}

impl Gadget<Global> for MissionGadget {
    fn name(&self) -> &'static str {
        "Mission"
    }

    fn game_frame(&self, _ctx: &spring_addons::AddonContext<'_, Global>, frame: i32) {
        if frame_consumed() {
            return;
        }
        orchestration::apply_mission(frame);
    }
}
