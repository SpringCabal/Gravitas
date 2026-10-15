use core::cell::Cell;

use spring_addons::Resource;

use crate::widgets::gfx_fire::fire::Fire;
use crate::widgets::gui_hud::hud::Hud;

#[derive(Default)]
pub struct Global {
    pub player_unit: Cell<Option<i32>>,
    pub hud: Resource<Hud>,
    pub clock: Cell<Clock>,
    pub fire: Resource<Fire>,
    #[cfg(feature = "headless-integration")]
    pub scripted_pointer: Cell<Option<crate::widgets::dbg_screenshots::ScriptedPointer>>,
}

#[derive(Clone, Copy, Default)]
pub struct Clock {
    pub frame: i32,
    pub elapsed: i32,
}
