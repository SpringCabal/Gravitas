use spring::ResultExt;
use spring_addons::event::{EventResult, KeyEvent};
use spring_addons::ui::Widget;

use core::cell::Cell;

use crate::input::{self, Input};
use crate::state::Global;

/// Keyboard and mouse: movement, aiming, shooting and jumping go to the rules
/// module; F11/F12 take screenshots; Ctrl+Q leaves.
#[derive(Default)]
pub struct GuiInputWidget {
    input: Input,
    /// The game frame input was last polled on.
    polled_frame: Cell<Option<u32>>,
}

impl Widget<Global> for GuiInputWidget {
    fn name(&self) -> &'static str {
        "GuiInput"
    }

    /// Poll once per game frame: the rules read the input in GameFrame.
    fn update(&self, ctx: &spring_addons::AddonContext<'_, Global>, _dt: f32) {
        let frame = spring::game::frame().ok();
        if frame.is_some() && self.polled_frame.replace(frame) != frame {
            self.input.poll(ctx.global());
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
        if event.matches("f11") || event.matches("f12") {
            spring::send_command("screenshot").log_err("screenshot key");
            return EventResult::Handled;
        }

        // Gravitas has no text-console UI.  Consume both return variants so
        // the engine cannot open a console that the WASM HUD cannot close.
        if event.matches("enter") {
            return EventResult::Handled;
        }

        if springcabal::menu::is_quit_to_menu(event) {
            springcabal::menu::quit_to_menu();
            return EventResult::Handled;
        }
        self.input.key(event, true).into()
    }

    fn key_release(
        &self,
        _ctx: &spring_addons::AddonContext<'_, Global>,
        event: &KeyEvent<'_>,
    ) -> EventResult {
        self.input.key(event, false).into()
    }
}
