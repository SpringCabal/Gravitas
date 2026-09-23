use core::cell::RefCell;

use alloc::string::String;

use spring_addons::AddonContext;
use spring_addons::rules::Gadget;

use crate::state::Global;
use crate::status;

const STATUS_PERIOD: i32 = 3;

/// Keeps the ui module's HUD up to date, sending the status when it changes.
#[derive(Default)]
pub struct StatusGadget {
    last: RefCell<String>,
}

impl Gadget<Global> for StatusGadget {
    fn name(&self) -> &'static str {
        "Status"
    }

    fn game_frame(&self, ctx: &AddonContext<'_, Global>, frame: i32) {
        if frame % STATUS_PERIOD != 0 {
            return;
        }
        let message = status::message(ctx.global(), frame);
        let mut last = self.last.borrow_mut();
        if *last != message {
            let _ = spring::send_to_unsynced(&message);
            *last = message;
        }
    }
}
