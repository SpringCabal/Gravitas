use core::cell::Cell;

use spring_addons::AddonContext;
use spring_addons::ui::Widget;

use crate::state::{Clock, Global};

const MAX_CATCH_UP_FRAMES: i32 = 30;

#[derive(Default)]
pub struct GameClockWidget {
    last_frame: Cell<i32>,
}

impl Widget<Global> for GameClockWidget {
    fn name(&self) -> &'static str {
        "GameClock"
    }

    fn update(&self, ctx: &AddonContext<'_, Global>, _dt: f32) {
        let frame = spring::game::current_frame();
        let elapsed = (frame - self.last_frame.replace(frame)).clamp(0, MAX_CATCH_UP_FRAMES);
        ctx.global().clock.set(Clock { frame, elapsed });
    }
}
