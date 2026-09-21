use spring_addons::register_gadgets;
use spring_addons::rules::{Gadget, GadgetHandler};

use super::*;
use crate::state::{self, Global};
use crate::{diagnostics, restart, terrain};

#[derive(Default)]
pub struct SetupGadget;

impl SetupGadget {
    pub fn new() -> Self {
        Self
    }
}

impl Gadget<Global> for SetupGadget {
    fn name(&self) -> &'static str {
        "Setup"
    }

    fn game_frame(&self, _ctx: &spring_addons::AddonContext<'_, Global>, frame: i32) {
        let _ = state::with_world(|world| {
            world.frame = frame;
            world.frame_consumed = false;
        });
        restart::finish_wipe();
        if restart::take() {
            restart::perform();
            diagnostics::info("restarting the level");
            let _ = state::with_world(|world| world.frame_consumed = true);
            return;
        }
        if crate::setup::take_setup_flag() {
            crate::setup::setup();
            let _ = state::with_world(|world| world.frame_consumed = true);
            return;
        }
        crate::setup::link();
        terrain::flush(frame);
    }
}

pub fn frame_consumed() -> bool {
    state::with_world(|world| world.frame_consumed).unwrap_or(false)
}

pub fn setup(handler: &mut GadgetHandler<Global>) {
    register_gadgets!(
        handler,
        [
            SetupGadget::new(),
            JumpGadget::new(),
            ElectrafiGadget::new(),
            MissionGadget::new(),
            PlatesGadget::new(),
            StatusGadget::new(),
            TestsGadget::new(),
            LifecycleGadget::new(),
            RobotsGadget::new(),
            InputGadget::new(),
        ]
    );
}
