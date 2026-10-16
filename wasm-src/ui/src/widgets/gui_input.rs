mod input;

use spring_addons::event::{EventResult, KeyEvent};
use spring_addons::ui::Widget;

use core::cell::Cell;

use self::input::Input;
use crate::state::Global;

#[derive(Default)]
pub struct GuiInputWidget {
    input: Input,
    polled_frame: Cell<Option<u32>>,
}

impl Widget<Global> for GuiInputWidget {
    fn name(&self) -> &'static str {
        "GuiInput"
    }

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
