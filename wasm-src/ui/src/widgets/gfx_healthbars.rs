use spring_addons::ui::Widget;

use crate::healthbars;
use crate::state::{self, Global};

#[derive(Default)]
pub struct GfxHealthbarsWidget;

impl GfxHealthbarsWidget {
    pub fn new() -> Self {
        Self
    }
}

impl Widget<Global> for GfxHealthbarsWidget {
    fn name(&self) -> &'static str {
        "GfxHealthbars"
    }

    fn draw_world(&self, _ctx: &spring_addons::AddonContext<'_, Global>) {
        let seconds = state::with_state(|ui| ui.seconds).unwrap_or(0.0);
        healthbars::draw(seconds);
    }
}
