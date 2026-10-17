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

    fn update(&self, ctx: &AddonContext<'_, Global, spring::Ui>, _dt: f32) {
        let spring = ctx.spring();
        let frame = spring.game().current_frame().unwrap_or(0);
        let elapsed = (frame - self.last_frame.replace(frame)).clamp(0, MAX_CATCH_UP_FRAMES);
        ctx.global().clock.set(Clock { frame, elapsed });
    }
}
