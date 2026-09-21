use core::cell::RefCell;

use crate::fire::Field;

#[derive(Clone, Copy, Default)]
pub struct HudBindings {
    pub health_percent: u64,
    pub health_caption: u64,
    pub health_color: u64,
    pub push_ready: u64,
    pub pull_ready: u64,
    pub paused: u64,
    pub endgame_visible: u64,
    pub endgame_victory: u64,
    pub endgame_title: u64,
}

pub struct UiState {
    pub context: u64,
    pub document: u64,
    pub hud: Option<HudBindings>,
    pub view_width: i32,
    pub view_height: i32,
    pub default_layout_hidden: bool,
    pub bindings_applied: bool,
    pub player_unit: i32,
    pub fire_pass: usize,
    pub last_game_frame: i32,
    pub frames_this_update: i32,
    pub camera_override: Option<(f32, f32, f32)>,
    pub scripted_target: Option<(f32, f32)>,
    pub move_north: bool,
    pub move_south: bool,
    pub move_west: bool,
    pub move_east: bool,
    pub jump: bool,
    pub suppressed_units: i32,
    pub endgame_open: bool,
    pub was_paused: bool,
    pub game_frame: i32,
    pub seconds: f32,
    pub edge_setup_failed: bool,
    pub edge_list: u32,
    pub edge_border_hidden: bool,
    pub camera_initialized: bool,
    pub camera_initial_updates: u8,
    pub fire: Field,
    pub fire_seed: u64,
    pub music_started: bool,
    pub music_time: f32,
    pub music_track_time: f32,
}

thread_local! {
    static STATE: RefCell<UiState> = RefCell::new(UiState::new());
}

pub fn with_state<R>(function: impl FnOnce(&mut UiState) -> R) -> Option<R> {
    STATE.with(|state| state.try_borrow_mut().ok().map(|mut ui| function(&mut ui)))
}

impl UiState {
    fn new() -> Self {
        Self {
            context: 0,
            document: 0,
            hud: None,
            view_width: 0,
            view_height: 0,
            default_layout_hidden: false,
            bindings_applied: false,
            player_unit: -1,
            fire_pass: 0,
            last_game_frame: 0,
            frames_this_update: 0,
            camera_override: None,
            scripted_target: None,
            move_north: false,
            move_south: false,
            move_west: false,
            move_east: false,
            jump: false,
            suppressed_units: -1,
            endgame_open: false,
            was_paused: false,
            game_frame: 0,
            seconds: 0.0,
            edge_setup_failed: false,
            edge_list: 0,
            edge_border_hidden: false,
            camera_initialized: false,
            camera_initial_updates: 0,
            fire: Field::new(),
            fire_seed: 0,
            music_started: false,
            music_time: 0.0,
            music_track_time: 0.0,
        }
    }

    pub fn move_vector(&self) -> (f32, f32) {
        let x = f32::from(self.move_east) - f32::from(self.move_west);
        let z = f32::from(self.move_south) - f32::from(self.move_north);
        (x, z)
    }
}

#[derive(Default)]
pub struct Global;
