use spring_addons::ui::Widget;

use crate::hud;
use crate::state::Global;

#[derive(Default)]
pub struct GuiHudWidget;

impl GuiHudWidget {
    pub fn new() -> Self {
        Self
    }
}

impl Widget<Global> for GuiHudWidget {
    fn name(&self) -> &'static str {
        "GuiHud"
    }

    fn update(&self, _ctx: &spring_addons::AddonContext<'_, Global>, _dt: f32) {
        hud::initialize();
        hud::hide_default_layout();
        hud::apply_bindings();
        if let Ok(geometry) = spring::display::get_view_geometry(0) {
            hud::render(geometry.view_size_x, geometry.view_size_y);
        }
    }
}

#[derive(Default)]
pub struct GuiHudPauseWidget;

impl GuiHudPauseWidget {
    pub fn new() -> Self {
        Self
    }
}

impl Widget<Global> for GuiHudPauseWidget {
    fn name(&self) -> &'static str {
        "GuiHudPause"
    }

    fn update(&self, _ctx: &spring_addons::AddonContext<'_, Global>, _dt: f32) {
        crate::selection::suppress();
        hud::update_pause();
    }
}
