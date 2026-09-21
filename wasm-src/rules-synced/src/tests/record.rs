use core::cell::RefCell;

use alloc::string::String;
use alloc::vec::Vec;

pub fn note_destroyed(unit_id: i32, unit_def_id: i32) {
    with_record(|record| {
        record.deaths += 1;
        if record.first_death.is_none() {
            record.first_death = Some(describe(unit_id, unit_def_id));
        }
    });
}

pub fn note_toggled(unit_id: i32, active: bool, _frame: i32) {
    with_record(|record| {
        if record.toggles.len() >= TOGGLE_CAPACITY {
            record.toggles.remove(0);
        }
        record.toggles.push((unit_id, active));
    });
}

pub fn deaths() -> u32 {
    with_record(|record| record.deaths).unwrap_or(0)
}

pub fn first_death() -> Option<String> {
    with_record(|record| record.first_death.clone()).flatten()
}

pub fn clear_toggles() {
    with_record(|record| record.toggles.clear());
}

pub fn toggled_to(unit_id: i32, active: bool) -> bool {
    with_record(|record| record.toggles.contains(&(unit_id, active))).unwrap_or(false)
}

const TOGGLE_CAPACITY: usize = 64;

struct Record {
    deaths: u32,
    first_death: Option<String>,
    toggles: Vec<(i32, bool)>,
}

thread_local! {
    static RECORD: RefCell<Record> = const {
        RefCell::new(Record {
            deaths: 0,
            first_death: None,
            toggles: Vec::new(),
        })
    };
}

fn with_record<R>(function: impl FnOnce(&mut Record) -> R) -> Option<R> {
    RECORD.with(|record| match record.try_borrow_mut() {
        Ok(mut record) => Some(function(&mut record)),
        Err(_) => None,
    })
}

fn describe(unit_id: i32, unit_def_id: i32) -> String {
    alloc::format!(
        "{} (unit {unit_id})",
        crate::units::def_name(unit_def_id).unwrap_or_else(|| String::from("?"))
    )
}
