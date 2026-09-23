//! Engine command ids, until the spring crate exposes CMD ids.

pub const STOP: i32 = 0;
pub const MOVE: i32 = 10;
pub const FIGHT: i32 = 16;
pub const FIRE_STATE: i32 = 45;
pub const MOVE_STATE: i32 = 50;
pub const ONOFF: i32 = 85;

/// `CMD_FIRE_STATE` parameter: hold fire.
pub const FIRE_STATE_HOLD: f32 = 0.0;
/// `CMD_MOVE_STATE` parameter: hold position.
#[cfg(feature = "e2e")]
pub const MOVE_STATE_HOLD: f32 = 0.0;
