use spring_addons::rules::Gadget;

use crate::gadgets::setup::frame_consumed;
use crate::state::Global;
use crate::tests;

#[derive(Default)]
pub struct TestsGadget;

impl TestsGadget {
    pub fn new() -> Self {
        Self
    }
}

impl Gadget<Global> for TestsGadget {
    fn name(&self) -> &'static str {
        "Tests"
    }

    fn game_frame(&self, _ctx: &spring_addons::AddonContext<'_, Global>, frame: i32) {
        if frame_consumed() {
            return;
        }
        tests::game_frame(frame);
    }
}
