use alloc::string::String;
use alloc::vec::Vec;

use crate::state::Global;

/// What the gadgets report to the e2e cases: deaths and plate/gate toggles.
#[derive(Default)]
pub struct Record {
    deaths: u32,
    first_death: Option<String>,
    toggles: Vec<(i32, bool)>,
    /// Fire and electric-field units and their size when UnitCreated reached
    /// the tests gadget.
    hazard_sizes: Vec<(i32, Option<f32>)>,
    /// Screenshots the ui module reported taken.
    shots: Vec<String>,
    /// Units that sparked in an electric field without being hurt.
    unhurt_sparks: Vec<i32>,
}

pub fn note_created(global: &Global, unit_id: i32, unit_def_id: i32) {
    let defs = crate::defs::get();
    let param = if defs.fire == Some(unit_def_id) {
        crate::fire::SIZE_PARAM
    } else if defs.electrafi == Some(unit_def_id) {
        crate::electrafi::SIZE_PARAM
    } else {
        return;
    };
    let size = spring::get_unit_rules_param_f32(unit_id, param).ok();
    global
        .tests
        .record
        .borrow_mut()
        .hazard_sizes
        .push((unit_id, size));
}

/// The hazard's size when it was created, if it was recorded (the latest
/// record: ids are reused after a restart).
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

/// A message from the ui module: `gravitas_e2e|shot|<name>` once it took a
/// screenshot.
pub fn note_message(global: &Global, data: &[u8]) {
    let Some(name) = data.strip_prefix(b"gravitas_e2e|shot|") else {
        return;
    };
    let Ok(name) = core::str::from_utf8(name) else {
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

/// An electric field's spark on `unit_id`, and whether it came with damage.
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
