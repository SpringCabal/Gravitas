// Compiled only for the external Lua/Rust visual comparison fixture.
use super::{Field, emit};
use crate::state;
use core::cell::Cell;
thread_local! {
    static STAGE: Cell<i32> = const { Cell::new(-1) };
    static SHOT: Cell<i32> = const { Cell::new(-1) };
}
pub fn update() {
    // Keep the engine cursor outside the comparison region.
    let _ = spring::unsynced_ctrl::warp_mouse(1100, 500);
    crate::camera::look_at(4288.0, 600.0, Some(650.0));
    let Ok(counter) = spring::game::get_game_frame(0) else {
        return;
    };
    let frame = (counter.low16 | counter.high16 << 16) as i32;
    if frame < 30 {
        return;
    }
    let stage = ((frame - 30) / 30).min(4);
    let target = [90, 105, 120, 150, 180][stage as usize];
    if STAGE.with(|v| v.replace(stage)) != stage {
        let _ = state::with_state(|ui| ui.fire = Field::new());
        for n in 30..=target {
            super::update(1.0);
            emit::raw_burn(
                spring::types::Float3 {
                    x: 4400.0,
                    y: 144.25,
                    z: 531.0,
                },
                25.0,
                1.0,
                [0.5, -0.25],
                (n * 20) as u64,
            );
            if n % 15 == 0 {
                emit::raw_flame(
                    spring::types::Float3 {
                        x: 4288.0,
                        y: 144.25,
                        z: 531.0,
                    },
                    500.0,
                    0.5,
                    (n * 10) as u64,
                );
            }
        }
    }
    let _ = state::with_state(|ui| ui.game_frame = target);
    if frame >= 45 + 30 * stage && SHOT.with(|v| v.replace(stage)) != stage {
        let _ = spring::send_commands("screenshot png", "");
    }
    if frame >= 190 {
        let _ = spring::send_commands("quitforce", "");
    }
}
