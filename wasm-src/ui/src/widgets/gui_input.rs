use spring_addons::event::{EventResult, KeyEvent};
use spring_addons::ui::Widget;

use crate::input;
use crate::state::{self, Global};

const MAX_CATCH_UP_FRAMES: i32 = 30;

#[derive(Default)]
pub struct GuiInputWidget;

impl GuiInputWidget {
    pub fn new() -> Self {
        Self
    }
}

impl Widget<Global> for GuiInputWidget {
    fn name(&self) -> &'static str {
        "GuiInput"
    }

    fn update(&self, _ctx: &spring_addons::AddonContext<'_, Global>, _dt: f32) {
        let frames = elapsed_frames();
        let _ = state::with_state(|ui| ui.frames_this_update = frames);
        if frames > 0 {
            input::poll();
        }
    }

    fn mouse_press(
        &self,
        _ctx: &spring_addons::AddonContext<'_, Global>,
        x: i32,
        y: i32,
        button: i32,
    ) -> EventResult {
        input::mouse_press(x, y, button).into()
    }

    fn mouse_release(
        &self,
        _ctx: &spring_addons::AddonContext<'_, Global>,
        x: i32,
        y: i32,
        button: i32,
    ) -> EventResult {
        input::mouse_release(x, y, button);
        EventResult::Handled
    }

    fn key_press(
        &self,
        _ctx: &spring_addons::AddonContext<'_, Global>,
        event: &KeyEvent<'_>,
    ) -> EventResult {
        // Keep screenshot capture out of the gameplay key path.  In
        // particular, this makes F12 reliable even after the HUD has removed
        // the engine's default key bindings.
        if event.key_code == KEY_F11 || event.key_code == KEY_F12 {
            let _ = spring::send_commands("screenshot", "");
            return EventResult::Handled;
        }

        // Gravitas has no text-console UI.  Consume both return variants so
        // the engine cannot open a console that the WASM HUD cannot close.
        if event.matches("enter") {
            return EventResult::Handled;
        }

        if event.key_code == KEY_CTRL_Q
            || (event.ctrl
                && (event.matches("q")
                    || event.matches("Q")
                    || event.key_code == KEY_Q
                    || event.key_code == KEY_Q_UPPER))
        {
            crate::hud::reload();
            return EventResult::Handled;
        }
        input::key_press(event.key_code).into()
    }

    fn key_release(
        &self,
        _ctx: &spring_addons::AddonContext<'_, Global>,
        event: &KeyEvent<'_>,
    ) -> EventResult {
        input::key_release(event.key_code).into()
    }
}

const KEY_F11: i32 = 292;
const KEY_F12: i32 = 293;
/// Ctrl+Q as some platforms report it: the ASCII control character DC1.
const KEY_CTRL_Q: i32 = 17;
const KEY_Q: i32 = b'q' as i32;
const KEY_Q_UPPER: i32 = b'Q' as i32;

fn elapsed_frames() -> i32 {
    let frame = spring::game::frame().map_or(0, |frame| frame as i32);
    state::with_state(|ui| {
        let elapsed = frame - ui.last_game_frame;
        ui.last_game_frame = frame;
        elapsed.clamp(0, MAX_CATCH_UP_FRAMES)
    })
    .unwrap_or(0)
}
