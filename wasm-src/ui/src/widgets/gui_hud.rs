use spring_addons::AddonContext;
use spring_addons::ui::Widget;

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
        hud::hide_default_layout(&mut hud);
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
        if let Some(outcome) = text.strip_prefix("GRAVITAS_OVER|") {
            hud::game_over(&mut global.hud.access_mut(), outcome == "1");
        } else if text.starts_with("GRAVITAS_STATUS|") {
            let player_unit = hud::update(&mut global.hud.access_mut(), text);
            global.player_unit.set(player_unit);
        }
    }

    fn game_over(&self, ctx: &AddonContext<'_, Global>, _winning_ally_teams: &[u8]) {
        hud::game_over(&mut ctx.global().hud.access_mut(), false);
    }
}
