use spring_addons::AddonContext;
use spring_addons::ui::Widget;

use crate::fx;
use crate::state::Global;

/// Turns the rules module's effect messages (burning units, flames, electric
/// arcs and sparks) into fire particles.
pub struct GfxRulesEffectsWidget;

impl Widget<Global> for GfxRulesEffectsWidget {
    fn name(&self) -> &'static str {
        "GfxRulesEffects"
    }

    fn recv_from_synced(&self, _ctx: &AddonContext<'_, Global>, message: &[u8]) {
        if message.starts_with(b"GRAVITAS_FX|")
            && let Ok(text) = core::str::from_utf8(message)
        {
            fx::handle(text);
        }
    }
}
