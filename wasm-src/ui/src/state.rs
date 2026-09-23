//! What the widgets share. Everything else is owned by the widget that uses
//! it.

use core::cell::{Cell, RefCell};

use spring_addons::Resource;

use crate::fire::Field;
use crate::hud::Hud;

#[derive(Default)]
pub struct Global {
    /// The unit the local player controls, as the rules report it.
    pub player_unit: Cell<Option<i32>>,
    /// The RmlUi HUD, also used by its event callbacks.
    pub hud: Resource<Hud>,
    /// Input and camera the e2e scenario drives instead of the player.
    pub scripted: Resource<Scripted>,
}

#[derive(Default)]
pub struct Scripted {
    pub camera: Option<(f32, f32, f32)>,
    pub target: Option<(f32, f32)>,
    pub jump: bool,
}

/// The particle state and the game clock the fire effects read.
pub struct UiState {
    /// The game frame at the start of this Update.
    pub game_frame: i32,
    /// Game frames since the previous Update.
    pub frames_this_update: i32,
    pub fire_pass: usize,
    pub fire: Field,
    pub fire_seed: u64,
}

thread_local! {
    static STATE: RefCell<UiState> = RefCell::new(UiState::new());
}

/// Borrow the fire and clock state. Panics on a conflicting borrow rather than
/// skipping the update.
pub fn with_state<R>(function: impl FnOnce(&mut UiState) -> R) -> Option<R> {
    STATE.with(|state| {
        let mut ui = state
            .try_borrow_mut()
            .expect("gravitas ui state is already borrowed");
        Some(function(&mut ui))
    })
}

impl UiState {
    fn new() -> Self {
        Self {
            game_frame: 0,
            frames_this_update: 0,
            fire_pass: 0,
            fire: Field::new(),
            fire_seed: 0,
        }
    }
}
