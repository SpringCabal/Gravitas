use crate::state::Global;

/// Without the `e2e` feature there is no test state.
#[derive(Default)]
pub struct Shared;

#[derive(Default)]
pub struct Runner;

impl Runner {
    pub fn game_frame(&self, _global: &Global, _frame: i32) {}
}

pub fn stage_level(_global: &Global) -> bool {
    false
}

pub fn note_created(_global: &Global, _unit_id: i32, _unit_def_id: i32) {}

pub fn note_destroyed(_global: &Global, _unit_id: i32, _unit_def_id: i32) {}

pub fn note_toggled(_global: &Global, _unit_id: i32, _active: bool) {}
