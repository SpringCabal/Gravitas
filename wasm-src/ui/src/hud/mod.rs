mod document;
mod model;
mod overlays;
mod status;

pub use document::{apply_bindings, initialize, render};
#[cfg(feature = "e2e")]
pub use model::restart;
pub use overlays::{game_over, update_pause};
pub use status::update;

/// The RmlUi HUD: health, gun readiness, pause and the end-of-game panel.
#[derive(Default)]
pub struct Hud {
    context: u64,
    bindings: Option<model::HudBindings>,
    keys_bound: bool,
    endgame_open: bool,
    was_paused: bool,
    /// The latest status, shown again once the HUD exists.
    status: Option<gravitas_shared::Status>,
}
