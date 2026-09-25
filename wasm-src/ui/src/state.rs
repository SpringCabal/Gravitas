//! What the widgets share. Everything else is owned by the widget that uses
//! it.

use core::cell::Cell;

use spring_addons::Resource;

use crate::fire::Fire;
use crate::hud::Hud;

#[derive(Default)]
pub struct Global {
    /// The unit the local player controls, as the rules report it.
    pub player_unit: Cell<Option<i32>>,
    /// The RmlUi HUD, also used by its event callbacks.
    pub hud: Resource<Hud>,
    /// The game clock, recorded by the GameClock widget at the start of each
    /// Update for the widgets that advance with the game.
    pub clock: Cell<Clock>,
    /// The fire and electric particles: the rules' effect messages spawn
    /// them, the fire widget moves and draws them.
    pub fire: Resource<Fire>,
    /// The e2e scenario's stand-in for the mouse: the input widget sends it
    /// through the normal input message.
    #[cfg(feature = "headless-integration")]
    pub scripted_pointer: Cell<Option<crate::shots::ScriptedPointer>>,
}

#[derive(Clone, Copy, Default)]
pub struct Clock {
    /// The game frame at the start of this Update.
    pub frame: i32,
    /// Game frames since the previous Update.
    pub elapsed: i32,
}
