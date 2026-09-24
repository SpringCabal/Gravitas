use spring_addons::AddonContext;
use spring_addons::ui::Widget;

use crate::camera::Camera;
use crate::state::Global;

#[derive(Default)]
pub struct GuiCameraWidget {
    camera: Camera,
}

impl Widget<Global> for GuiCameraWidget {
    fn name(&self) -> &'static str {
        "GuiCamera"
    }

    /// Don't wait for the first status message: the engine already knows the
    /// player's unit during ui startup.
    fn init(&self, ctx: &AddonContext<'_, Global>) {
        self.camera.follow(ctx.global());
    }

    fn update(&self, ctx: &AddonContext<'_, Global>, _dt: f32) {
        self.camera.follow(ctx.global());
    }

    /// The first status message can come before the first Update; follow it
    /// so the full-map camera never shows.
    fn recv_from_synced(&self, ctx: &AddonContext<'_, Global>, message: &[u8]) {
        if message.starts_with(b"GRAVITAS_STATUS|") {
            self.camera.follow(ctx.global());
        }
        #[cfg(feature = "e2e")]
        if message.starts_with(b"GRAVITAS_SHOT|")
            && let Ok(text) = core::str::from_utf8(message)
        {
            self.camera.handle_shot_message(text);
        }
    }
}
