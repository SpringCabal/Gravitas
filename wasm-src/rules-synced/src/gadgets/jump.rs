use spring_addons::rules::Gadget;

use crate::gadgets::setup::frame_consumed;
use crate::orchestration;
use crate::probe;
use crate::state::{self, Global};

#[derive(Default)]
pub struct JumpGadget;

impl JumpGadget {
    pub fn new() -> Self {
        Self
    }
}

impl Gadget<Global> for JumpGadget {
    fn name(&self) -> &'static str {
        "Jump"
    }

    fn game_frame(&self, _ctx: &spring_addons::AddonContext<'_, Global>, frame: i32) {
        if frame_consumed() {
            return;
        }
        let _ = state::with_world(|world| probe::report_late(world, frame));
        let _ = state::with_world(|world| world.player.update(frame));
        orchestration::start_jump();
        orchestration::land_jumps(frame);
        let _ = state::with_world(|world| world.impulses.apply());
    }
}
