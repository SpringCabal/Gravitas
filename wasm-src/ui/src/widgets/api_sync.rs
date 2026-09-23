use spring_addons::AddonContext;
use spring_addons::ui::Widget;

use crate::state::Global;
use crate::{fx, shots};

/// Effects and e2e scenario messages from the rules module.
pub struct ApiSyncWidget;

impl Widget<Global> for ApiSyncWidget {
    fn name(&self) -> &'static str {
        "ApiSync"
    }

    fn recv_from_synced(&self, ctx: &AddonContext<'_, Global>, message: &[u8]) {
        let Ok(text) = core::str::from_utf8(message) else {
            return;
        };
        if text.starts_with("GRAVITAS_SHOT|") {
            shots::handle(ctx.global(), text);
        } else if text.starts_with("GRAVITAS_FX|") {
            fx::handle(text);
        }
    }
}
