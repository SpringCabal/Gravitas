use crate::tests::harness::{self, Case, Step, fail};
use crate::tests::record;
use crate::units;

pub struct Spawned;

impl Spawned {
    pub fn new() -> Self {
        Self
    }
}

impl Case for Spawned {
    fn name(&self) -> &'static str {
        "spawn"
    }

    fn step(&mut self, frame: i32) -> Step {
        if frame < VERDICT_FRAME {
            return Step::Wait;
        }
        let Some(units) = harness::arena(|arena| arena.units()) else {
            return fail("no arena");
        };
        let missing = units
            .iter()
            .filter(|unit_id| !units::valid(**unit_id))
            .count();
        if missing != 0 {
            return Step::Fail(alloc::format!(
                "{missing} of {} rig units missing",
                units.len()
            ));
        }
        if record::deaths() != 0 {
            return Step::Fail(alloc::format!(
                "{} unit(s) died during setup, first {}",
                record::deaths(),
                record::first_death().unwrap_or_default()
            ));
        }
        Step::Pass
    }
}

const VERDICT_FRAME: i32 = 1;
