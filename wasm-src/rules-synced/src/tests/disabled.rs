use crate::state::Global;

pub fn stage_level(_global: &Global) -> bool {
    false
}

pub fn game_frame(_global: &Global, _frame: i32) {}

pub fn note_destroyed(_unit_id: i32, _unit_def_id: i32) {}

pub fn note_toggled(_unit_id: i32, _active: bool, _frame: i32) {}
