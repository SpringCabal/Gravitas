pub mod fire;
use spring_addons::AddonContext;
use spring_addons::ui::Widget;

use self::fire::Distortion;
use crate::state::Global;

#[derive(Default)]
pub struct GfxFireWidget {
    distortion: Distortion,
    #[cfg(feature = "fire-probe")]
    probe: fire::probe::Probe,
}

impl GfxFireWidget {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Widget<Global> for GfxFireWidget {
    fn name(&self) -> &'static str {
        "GfxFire"
    }

    fn update(&self, ctx: &AddonContext<'_, Global, spring::Ui>, _dt: f32) {
        let _spring = ctx.spring();
        let global = ctx.global();
        #[cfg(feature = "fire-probe")]
        self.probe.update(_spring, global);
        #[cfg(not(feature = "fire-probe"))]
        {
            let frames = global.clock.get().elapsed;
            if frames > 0 {
                global.fire.access_mut().advance(frames as f32);
            }
        }
    }

    fn draw_world(&self, ctx: &AddonContext<'_, Global, spring::Ui>) {
        let spring = ctx.spring();
        let global = ctx.global();
        let time = global.clock.get().frame as f32;
        fire::draw(spring, &global.fire.access(), &self.distortion, time);
    }
}
