use spring_addons::ui::Widget;

use crate::camera;
use crate::state::Global;
use crate::{fx, hud, shots};

#[derive(Default)]
pub struct ApiSyncWidget;

impl ApiSyncWidget {
    pub fn new() -> Self {
        Self
    }
}

impl Widget<Global> for ApiSyncWidget {
    fn name(&self) -> &'static str {
        "ApiSync"
    }

    fn recv_from_synced(&self, _ctx: &spring_addons::AddonContext<'_, Global>, message: &[u8]) {
        let Ok(text) = core::str::from_utf8(message) else {
            return;
        };
        if text.starts_with("GRAVITAS_SHOT|") {
            shots::handle(text);
            return;
        }
        if text.starts_with("GRAVITAS_OVER|") {
            hud::game_over(text.ends_with('1'));
        } else if text.starts_with("GRAVITAS_STATUS|") {
            hud::update(text);
            // This can be the first callback after the game starts. Keep the
            // camera from showing the full map while waiting for Update().
            camera::follow();
        } else if text.starts_with("GRAVITAS_FX|") {
            fx::handle(text);
        }
    }

    fn game_over(
        &self,
        _ctx: &spring_addons::AddonContext<'_, Global>,
        _winning_ally_teams: &[u8],
    ) {
        hud::game_over(false);
    }
}
