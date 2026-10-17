use crate::state::Global;

#[derive(Default)]
pub struct Shared;

#[derive(Default)]
pub struct Runner;

impl Runner {
    pub fn game_frame(
        &self,
        _spring: &spring::Spring<'_, spring::RulesSynced>,
        _global: &Global,
        _frame: i32,
    ) {
    }
}

pub fn stage_level(_spring: &spring::Spring<'_, spring::RulesSynced>, _global: &Global) -> bool {
    false
}

pub fn note_created(
    _spring: &spring::Spring<'_, spring::RulesSynced>,
    _global: &Global,
    _unit_id: i32,
    _unit_def_id: i32,
) {
}

pub fn note_message(_global: &Global, _data: &[u8]) {}

pub fn note_destroyed(
    _spring: &spring::Spring<'_, spring::RulesSynced>,
    _global: &Global,
    _unit_id: i32,
    _unit_def_id: i32,
) {
}

pub fn note_toggled(_global: &Global, _unit_id: i32, _active: bool) {}

pub fn note_spark(_global: &Global, _unit_id: i32, _hurt: bool) {}
