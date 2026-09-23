/// The current game frame.
pub fn current() -> i32 {
    spring::game::get_game_frame(0)
        .map(|counter| (counter.low16 | (counter.high16 << 16)) as i32)
        .unwrap_or(0)
}
