use spring_addons::AddonContext;
use spring_addons::rules::Gadget;

use crate::state::Global;
use crate::status;

const STATUS_PERIOD: i32 = 3;

/// Keeps the ui module's HUD up to date.
pub struct StatusGadget;

impl Gadget<Global> for StatusGadget {
    fn name(&self) -> &'static str {
        "Status"
    }

    fn game_frame(&self, ctx: &AddonContext<'_, Global>, frame: i32) {
        if frame % STATUS_PERIOD == 0 {
            status::publish(ctx.global(), frame);
        }
    }
}
