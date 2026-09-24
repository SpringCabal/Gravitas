#[cfg(feature = "e2e")]
mod arena;
#[cfg(feature = "e2e")]
mod barrel;
#[cfg(feature = "e2e")]
mod death;
#[cfg(feature = "e2e")]
mod electrafi;
#[cfg(feature = "e2e")]
mod falling;
#[cfg(feature = "e2e")]
mod fire;
#[cfg(feature = "e2e")]
mod flee;
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
mod level;
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
mod shoot;
#[cfg(feature = "e2e")]
mod spawn;
#[cfg(feature = "e2e")]
mod terrain;
#[cfg(feature = "e2e")]
mod visuals;

#[cfg(not(feature = "e2e"))]
mod disabled;

#[cfg(feature = "e2e")]
pub use record::{note_created, note_destroyed, note_message, note_toggled};
#[cfg(feature = "e2e")]
pub use runner::{Runner, stage_level};

#[cfg(not(feature = "e2e"))]
pub use disabled::{
    Runner, Shared, note_created, note_destroyed, note_message, note_toggled, stage_level,
};

/// What the e2e cases share with the gadgets: the arena that replaces the
/// level, and what the gadgets report.
#[cfg(feature = "e2e")]
#[derive(Default)]
pub struct Shared {
    arena: core::cell::Cell<Option<arena::Arena>>,
    /// Link the arena's plate and gate on the next frame.
    pending_link: core::cell::Cell<bool>,
    /// Spawn the real level instead of the arena (the level case).
    real_level: core::cell::Cell<bool>,
    record: core::cell::RefCell<record::Record>,
}
