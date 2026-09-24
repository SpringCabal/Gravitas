use core::cell::Cell;

use spring_addons::AddonContext;
use spring_addons::ui::Widget;

use crate::state::{Clock, Global};

/// At most this many game frames are simulated in one Update after a stall.
const MAX_CATCH_UP_FRAMES: i32 = 30;

/// Runs first in Update: records the game frame and how many frames passed
/// since the last Update, for the widgets that advance with the game.
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
