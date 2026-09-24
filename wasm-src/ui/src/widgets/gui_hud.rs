use spring_addons::AddonContext;
use spring_addons::ui::Widget;

use gravitas_shared::Status;

use crate::hud;
use crate::state::Global;

/// The RmlUi HUD: health, gun readiness, pause and the end-of-game panel.
pub struct GuiHudWidget;

impl Widget<Global> for GuiHudWidget {
    fn name(&self) -> &'static str {
        "GuiHud"
    }

    fn update(&self, ctx: &AddonContext<'_, Global>, _dt: f32) {
        let mut hud = ctx.global().hud.access_mut();
        hud::initialize(&mut hud);
        hud::apply_bindings(&mut hud);
        if let Ok(geometry) = spring::display::get_view_geometry(0) {
            hud::render(&hud, geometry.view_size_x, geometry.view_size_y);
        }
        hud::update_pause(&mut hud);
    }

    fn recv_from_synced(&self, ctx: &AddonContext<'_, Global>, message: &[u8]) {
        let Ok(text) = core::str::from_utf8(message) else {
            return;
        };
        let global = ctx.global();
        if let Some(victory) = gravitas_shared::parse_game_over(text) {
            hud::game_over(&mut global.hud.access_mut(), victory);
        } else if let Some(status) = Status::parse(text) {
            hud::update(&mut global.hud.access_mut(), status);
            global.player_unit.set(status.player_unit);
        }
    }

    fn game_over(&self, ctx: &AddonContext<'_, Global>, _winning_ally_teams: &[u8]) {
        hud::game_over(&mut ctx.global().hud.access_mut(), false);
    }
}
