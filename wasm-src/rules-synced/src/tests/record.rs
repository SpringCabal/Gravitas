use alloc::string::String;
use alloc::vec::Vec;

use crate::state::Global;

/// What the gadgets report to the e2e cases: deaths and plate/gate toggles.
#[derive(Default)]
pub struct Record {
    deaths: u32,
    first_death: Option<String>,
    toggles: Vec<(i32, bool)>,
}

pub fn note_destroyed(global: &Global, unit_id: i32, unit_def_id: i32) {
    let mut record = global.tests.record.borrow_mut();
    record.deaths += 1;
    if record.first_death.is_none() {
        record.first_death = Some(describe(unit_id, unit_def_id));
    }
}

pub fn note_toggled(global: &Global, unit_id: i32, active: bool) {
    let mut record = global.tests.record.borrow_mut();
    if record.toggles.len() >= TOGGLE_CAPACITY {
        record.toggles.remove(0);
    }
    record.toggles.push((unit_id, active));
}

pub fn deaths(global: &Global) -> u32 {
    global.tests.record.borrow().deaths
}

pub fn first_death(global: &Global) -> Option<String> {
    global.tests.record.borrow().first_death.clone()
}

pub fn clear_toggles(global: &Global) {
    global.tests.record.borrow_mut().toggles.clear();
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

fn describe(unit_id: i32, unit_def_id: i32) -> String {
    alloc::format!(
        "{} (unit {unit_id})",
        crate::defs::get().unit(unit_def_id).name
    )
}
