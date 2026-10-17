pub(crate) mod camera;

use spring_addons::AddonContext;
use spring_addons::ui::Widget;

use crate::state::Global;
use crate::widgets::gui_camera::camera::Camera;

#[derive(Default)]
pub struct GuiCameraWidget {
    camera: Camera,
}

impl Widget<Global> for GuiCameraWidget {
    fn name(&self) -> &'static str {
        "GuiCamera"
    }

    fn init(&self, ctx: &AddonContext<'_, Global, spring::Ui>) {
        let spring = ctx.spring();
        self.camera.follow(spring, ctx.global(), 0.0);
    }

    fn update(&self, ctx: &AddonContext<'_, Global, spring::Ui>, dt: f32) {
        let spring = ctx.spring();
        self.camera.follow(spring, ctx.global(), dt);
    }

    fn recv_from_synced(&self, ctx: &AddonContext<'_, Global, spring::Ui>, message: &[u8]) {
        let spring = ctx.spring();
        if core::str::from_utf8(message)
            .ok()
            .and_then(gravitas_shared::Status::parse)
            .is_some()
        {
            self.camera.follow(spring, ctx.global(), 0.0);
        }
        #[cfg(feature = "headless-integration")]
        self.camera.handle_request(message);
    }
}
