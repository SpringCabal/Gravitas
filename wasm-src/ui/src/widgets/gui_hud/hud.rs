mod document;
mod model;
mod overlays;
mod status;

pub use document::{apply_bindings, initialize, render, shutdown};
#[cfg(feature = "headless-integration")]
pub use model::restart;
pub use overlays::{game_over, update_pause};
pub use status::update;

#[derive(Default)]
pub struct Hud {
    context: u64,
    bindings: Option<model::HudBindings>,
    keys_bound: bool,
    endgame: Option<bool>,
    was_paused: bool,
    status: Option<gravitas_shared::Status>,
}
