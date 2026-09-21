use spring_addons::rules::Gadget;

use crate::gadgets::setup::frame_consumed;
use crate::orchestration;
use crate::state::Global;

#[derive(Default)]
pub struct PlatesGadget;

impl PlatesGadget {
    pub fn new() -> Self {
        Self
    }
}

impl Gadget<Global> for PlatesGadget {
    fn name(&self) -> &'static str {
        "Plates"
    }

    fn game_frame(&self, _ctx: &spring_addons::AddonContext<'_, Global>, frame: i32) {
        if frame_consumed() {
            return;
        }
        orchestration::apply_plate_changes(frame);
    }
}
