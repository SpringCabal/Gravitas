use core::cell::Cell;

use spring_addons::AddonContext;
use spring_addons::ui::Widget;

use crate::healthbars;
use crate::state::Global;

#[derive(Default)]
pub struct GfxHealthbarsWidget {
    /// Wall-clock time, for the blink of a stunned unit's bar.
    seconds: Cell<f32>,
}

impl GfxHealthbarsWidget {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Widget<Global> for GfxHealthbarsWidget {
    fn name(&self) -> &'static str {
        "GfxHealthbars"
    }

    fn update(&self, _ctx: &AddonContext<'_, Global>, dt: f32) {
        self.seconds.set(self.seconds.get() + dt.max(0.0));
    }

    fn draw_world(&self, _ctx: &AddonContext<'_, Global>) {
        healthbars::draw(self.seconds.get());
    }
}
