use alloc::string::String;

use spring_addons::AddonContext;
use spring_addons::rules::Gadget;
use springcabal::message::SendOnChange;

use crate::state::Global;
use crate::status;

const STATUS_PERIOD: i32 = 3;

/// Keeps the ui module's HUD up to date, sending the status when it changes.
#[derive(Default)]
pub struct StatusGadget {
    sent: SendOnChange<String>,
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
        self.sent
            .send_if_changed(message, |message| springcabal::message::send_to_ui(message));
    }
}
