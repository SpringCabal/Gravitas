use alloc::string::String;
use alloc::vec::Vec;

use crate::state::Global;

#[derive(Default)]
pub struct Record {
    deaths: u32,
    first_death: Option<String>,
    destroyed: Vec<i32>,
    toggles: Vec<(i32, bool)>,
    hazard_sizes: Vec<(i32, Option<f32>)>,
    shots: Vec<String>,
    unhurt_sparks: Vec<i32>,
}

pub fn note_created(
    spring: &spring::Spring<'_, spring::RulesSynced>,
    global: &Global,
    unit_id: i32,
    unit_def_id: i32,
) {
    let defs = crate::defs::get(spring);
    let param = if defs.fire == Some(unit_def_id) {
        crate::gadgets::fire::behavior::SIZE_PARAM
    } else if defs.electrafi == Some(unit_def_id) {
        crate::gadgets::electrafi::behavior::SIZE_PARAM
    } else {
        return;
    };
    let size = spring
        .rules_params()
        .get_unit_rules_param_f32(unit_id, param)
        .ok();
    global
        .tests
        .record
        .borrow_mut()
        .hazard_sizes
        .push((unit_id, size));
}

pub fn hazard_size_at_creation(global: &Global, unit_id: i32) -> Option<Option<f32>> {
    global
        .tests
        .record
        .borrow()
        .hazard_sizes
        .iter()
        .rev()
        .find(|(id, _)| *id == unit_id)
        .map(|(_, size)| *size)
}

pub fn note_message(global: &Global, data: &[u8]) {
    let Some(name) = core::str::from_utf8(data)
        .ok()
        .and_then(gravitas_shared::e2e::parse_shot_taken)
    else {
        return;
    };
    global
        .tests
        .record
        .borrow_mut()
        .shots
        .push(String::from(name));
}

pub fn shot_taken(global: &Global, name: &str) -> bool {
    global
        .tests
        .record
        .borrow()
        .shots
        .iter()
        .any(|shot| shot == name)
}

pub fn note_destroyed(
    spring: &spring::Spring<'_, spring::RulesSynced>,
    global: &Global,
    unit_id: i32,
    unit_def_id: i32,
) {
    let mut record = global.tests.record.borrow_mut();
    record.deaths += 1;
    record.destroyed.push(unit_id);
    if record.first_death.is_none() {
        record.first_death = Some(describe(spring, unit_id, unit_def_id));
    }
}

pub fn note_toggled(global: &Global, unit_id: i32, active: bool) {
    let mut record = global.tests.record.borrow_mut();
    if record.toggles.len() >= TOGGLE_CAPACITY {
        record.toggles.remove(0);
    }
    record.toggles.push((unit_id, active));
}

pub fn note_spark(global: &Global, unit_id: i32, hurt: bool) {
    let mut record = global.tests.record.borrow_mut();
    if !hurt && !record.unhurt_sparks.contains(&unit_id) {
        record.unhurt_sparks.push(unit_id);
    }
}

pub fn sparked_unhurt(global: &Global, unit_id: i32) -> bool {
    global
        .tests
        .record
        .borrow()
        .unhurt_sparks
        .contains(&unit_id)
}

pub fn deaths(global: &Global) -> u32 {
    global.tests.record.borrow().deaths
}

pub fn first_death(global: &Global) -> Option<String> {
    global.tests.record.borrow().first_death.clone()
}

pub fn destroyed(global: &Global, unit_id: i32) -> bool {
    global.tests.record.borrow().destroyed.contains(&unit_id)
}

pub fn destroyed_count(global: &Global) -> usize {
    global.tests.record.borrow().destroyed.len()
}

pub fn destroyed_since(global: &Global, mark: usize, unit_id: i32) -> bool {
    global.tests.record.borrow().destroyed[mark..].contains(&unit_id)
}

pub fn clear_toggles(global: &Global, units: &[i32]) {
    global
        .tests
        .record
        .borrow_mut()
        .toggles
        .retain(|(unit_id, _)| !units.contains(unit_id));
}

pub fn toggled_to(global: &Global, unit_id: i32, active: bool) -> bool {
    global
        .tests
        .record
        .borrow()
        .toggles
        .contains(&(unit_id, active))
}

const TOGGLE_CAPACITY: usize = 64;

fn describe(
    spring: &spring::Spring<'_, spring::RulesSynced>,
    unit_id: i32,
    unit_def_id: i32,
) -> String {
    alloc::format!(
        "{} (unit {unit_id})",
        crate::defs::get(spring).unit(unit_def_id).name
    )
}
