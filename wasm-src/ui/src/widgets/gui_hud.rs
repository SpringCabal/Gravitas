pub mod hud;
use spring_addons::AddonContext;
use spring_addons::ui::Widget;

use gravitas_shared::Status;

use crate::state::Global;

pub struct GuiHudWidget;

impl Widget<Global> for GuiHudWidget {
    fn name(&self) -> &'static str {
        "GuiHud"
    }

    fn shutdown(&self, ctx: &AddonContext<'_, Global, spring::Ui>) {
        let spring = ctx.spring();
        hud::shutdown(spring, &mut ctx.global().hud.access_mut());
    }

    fn update(&self, ctx: &AddonContext<'_, Global, spring::Ui>, _dt: f32) {
        let spring = ctx.spring();
        let mut hud = ctx.global().hud.access_mut();
        hud::initialize(spring, &mut hud);
        if let Ok(geometry) = spring.display().get_view_geometry() {
            hud::render(spring, &hud, geometry.view_size_x, geometry.view_size_y);
        }
        hud::update_pause(spring, &mut hud);
    }

    fn recv_from_synced(&self, ctx: &AddonContext<'_, Global, spring::Ui>, message: &[u8]) {
        let spring = ctx.spring();
        let Ok(text) = core::str::from_utf8(message) else {
            return;
        };
        let global = ctx.global();
        if let Some(victory) = gravitas_shared::parse_game_over(text) {
            hud::game_over(spring, &mut global.hud.access_mut(), victory);
        } else if let Some(status) = Status::parse(text) {
            hud::update(spring, &mut global.hud.access_mut(), status);
            global.player_unit.set(status.player_unit);
        }
    }

    fn game_over(&self, ctx: &AddonContext<'_, Global, spring::Ui>, _winning_ally_teams: &[u8]) {
        let spring = ctx.spring();
        hud::game_over(spring, &mut ctx.global().hud.access_mut(), false);
    }
}
