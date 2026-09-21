use spring_addons::rules::Gadget;

use crate::gadgets::setup::frame_consumed;
use crate::state::{self, Global};
use crate::status;

const STATUS_PERIOD: i32 = 3;

#[derive(Default)]
pub struct StatusGadget;

impl StatusGadget {
    pub fn new() -> Self {
        Self
    }
}

impl Gadget<Global> for StatusGadget {
    fn name(&self) -> &'static str {
        "Status"
    }

    fn game_frame(&self, _ctx: &spring_addons::AddonContext<'_, Global>, frame: i32) {
        if frame_consumed() {
            return;
        }
        if frame % STATUS_PERIOD == 0 {
            let _ = state::with_world(|world| status::publish(world, frame));
        }
    }
}
