use spring_addons::AddonContext;
use spring_addons::ui::Widget;

use crate::shots;
use crate::state::Global;

/// Takes the e2e scenario's screenshots, plays its scripted input and quits
/// when it is done (only with the `e2e` feature).
pub struct DbgScreenshotsWidget;

impl Widget<Global> for DbgScreenshotsWidget {
    fn name(&self) -> &'static str {
        "DbgScreenshots"
    }

    fn recv_from_synced(&self, ctx: &AddonContext<'_, Global>, message: &[u8]) {
        shots::handle(ctx.global(), message);
    }
}
