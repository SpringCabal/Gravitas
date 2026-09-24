use spring_addons::ui::Widget;

use crate::fire;
#[cfg(not(feature = "fire-probe"))]
use crate::state;
use crate::state::Global;

#[derive(Default)]
pub struct GfxFireWidget;

impl GfxFireWidget {
    pub fn new() -> Self {
        Self
    }
}

impl Widget<Global> for GfxFireWidget {
    fn name(&self) -> &'static str {
        "GfxFire"
    }

    fn update(&self, _ctx: &spring_addons::AddonContext<'_, Global>, _dt: f32) {
        #[cfg(feature = "fire-probe")]
        {
            fire::probe::update();
        }
        #[cfg(not(feature = "fire-probe"))]
        {
            let frames = state::with_state(|ui| ui.frames_this_update).unwrap_or(0);
            if frames > 0 {
                fire::update(frames as f32);
            }
        }
    }

    fn draw_world(&self, _ctx: &spring_addons::AddonContext<'_, Global>) {
        fire::draw();
    }
}
