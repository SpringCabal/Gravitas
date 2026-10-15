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

    fn init(&self, ctx: &AddonContext<'_, Global>) {
        self.camera.follow(ctx.global(), 0.0);
    }

    fn update(&self, ctx: &AddonContext<'_, Global>, dt: f32) {
        self.camera.follow(ctx.global(), dt);
    }

    fn recv_from_synced(&self, ctx: &AddonContext<'_, Global>, message: &[u8]) {
        if core::str::from_utf8(message)
            .ok()
            .and_then(gravitas_shared::Status::parse)
            .is_some()
        {
            self.camera.follow(ctx.global(), 0.0);
        }
        #[cfg(feature = "headless-integration")]
        self.camera.handle_request(message);
    }
}
