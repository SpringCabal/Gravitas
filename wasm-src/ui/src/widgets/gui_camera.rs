use spring_addons::ui::Widget;

use crate::camera;
use crate::state::Global;

#[derive(Default)]
pub struct GuiCameraWidget;

impl GuiCameraWidget {
    pub fn new() -> Self {
        Self
    }
}

impl Widget<Global> for GuiCameraWidget {
    fn name(&self) -> &'static str {
        "GuiCamera"
    }

    fn init(&self, _ctx: &spring_addons::AddonContext<'_, Global>) {
        camera::initialize();
    }

    fn update(&self, _ctx: &spring_addons::AddonContext<'_, Global>, _dt: f32) {
        camera::follow();
    }
}
