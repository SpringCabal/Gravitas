use crate::state::Global;
use crate::tests::harness::{self, Case, Phase, Step, fail};
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

    fn phase(&self) -> Phase {
        Phase::Setup
    }

    fn step(
        &mut self,
        spring: &spring::Spring<'_, spring::RulesSynced>,
        global: &Global,
        frame: i32,
    ) -> Step {
        if frame < VERDICT_FRAME {
            return Step::Wait;
        }
        let Some(units) = harness::arena(global, |arena| arena.units()) else {
            return fail("no arena");
        };
        let missing = units
            .iter()
            .filter(|unit_id| !units::valid(spring, **unit_id))
            .count();
        if missing != 0 {
            return Step::Fail(alloc::format!(
                "{missing} of {} rig units missing",
                units.len()
            ));
        }
        if record::deaths(global) != 0 {
            return Step::Fail(alloc::format!(
                "{} unit(s) died during setup, first {}",
                record::deaths(global),
                record::first_death(global).unwrap_or_default()
            ));
        }
        Step::Pass
    }
}

const VERDICT_FRAME: i32 = 1;
