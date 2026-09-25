#[cfg(feature = "headless-integration")]
mod arena;
#[cfg(feature = "headless-integration")]
mod barrel;
#[cfg(feature = "headless-integration")]
mod collision;
#[cfg(feature = "headless-integration")]
mod death;
#[cfg(feature = "headless-integration")]
mod electrafi;
#[cfg(feature = "headless-integration")]
mod falling;
#[cfg(feature = "headless-integration")]
mod fire;
#[cfg(feature = "headless-integration")]
mod flee;
#[cfg(feature = "headless-integration")]
mod harness;
#[cfg(feature = "headless-integration")]
mod impulse;
#[cfg(feature = "headless-integration")]
mod input;
#[cfg(feature = "headless-integration")]
mod invulnerable;
#[cfg(feature = "headless-integration")]
mod jump;
#[cfg(feature = "headless-integration")]
mod level;
#[cfg(feature = "headless-integration")]
mod plate;
#[cfg(feature = "headless-integration")]
mod record;
#[cfg(feature = "headless-integration")]
mod respawn;
#[cfg(feature = "headless-integration")]
mod restarting;
#[cfg(feature = "headless-integration")]
mod runner;
#[cfg(feature = "headless-integration")]
mod shoot;
#[cfg(feature = "headless-integration")]
mod spawn;
#[cfg(feature = "headless-integration")]
mod terrain;
#[cfg(feature = "headless-integration")]
mod visuals;

#[cfg(not(feature = "headless-integration"))]
mod disabled;

#[cfg(feature = "headless-integration")]
pub use record::{note_created, note_destroyed, note_message, note_spark, note_toggled};
#[cfg(feature = "headless-integration")]
pub use runner::{Runner, stage_level};

#[cfg(not(feature = "headless-integration"))]
pub use disabled::{
    Runner, Shared, note_created, note_destroyed, note_message, note_spark, note_toggled,
    stage_level,
};

/// What the e2e cases share with the gadgets: the arena that replaces the
/// level, and what the gadgets report.
#[cfg(feature = "headless-integration")]
#[derive(Default)]
pub struct Shared {
    arena: core::cell::Cell<Option<arena::Arena>>,
    /// Link the arena's plate and gate on the next frame.
    pending_link: core::cell::Cell<bool>,
    /// Spawn the real level instead of the arena (the level case).
    real_level: core::cell::Cell<bool>,
    record: core::cell::RefCell<record::Record>,
    /// Where the cases that passed left their units, for the scene case.
    rests: core::cell::RefCell<alloc::vec::Vec<harness::Rest>>,
}
