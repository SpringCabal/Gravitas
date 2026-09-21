#[cfg(feature = "e2e")]
mod arena;
#[cfg(feature = "e2e")]
mod barrel;
#[cfg(feature = "e2e")]
mod electrafi;
#[cfg(feature = "e2e")]
mod falling;
#[cfg(feature = "e2e")]
mod fire;
#[cfg(feature = "e2e")]
mod harness;
#[cfg(feature = "e2e")]
mod impulse;
#[cfg(feature = "e2e")]
mod input;
#[cfg(feature = "e2e")]
mod invulnerable;
#[cfg(feature = "e2e")]
mod jump;
#[cfg(feature = "e2e")]
mod plate;
#[cfg(feature = "e2e")]
mod record;
#[cfg(feature = "e2e")]
mod respawn;
#[cfg(feature = "e2e")]
mod restarting;
#[cfg(feature = "e2e")]
mod runner;
#[cfg(feature = "e2e")]
mod spawn;
#[cfg(feature = "e2e")]
mod terrain;
#[cfg(feature = "e2e")]
mod visuals;

#[cfg(not(feature = "e2e"))]
mod disabled;

#[cfg(feature = "e2e")]
pub use record::{note_destroyed, note_toggled};
#[cfg(feature = "e2e")]
pub use runner::{game_frame, stage_level};

#[cfg(not(feature = "e2e"))]
pub use disabled::{game_frame, note_destroyed, note_toggled, stage_level};
