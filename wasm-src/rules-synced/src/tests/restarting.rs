use crate::mission;
use crate::tests::harness::{self, Case, Step, fail};
use crate::units;

pub struct Restarting {
    jumper: i32,
}

impl Restarting {
    pub fn new() -> Self {
        Self { jumper: -1 }
    }
}

impl Case for Restarting {
    fn name(&self) -> &'static str {
        "restart"
    }

    fn step(&mut self, frame: i32) -> Step {
        match frame {
            KILL_FRAME => self.kill(),
            PRESS_FRAME => self.press(),
            VERDICT_FRAME => self.check(),
            DIE_AGAIN_FRAME => self.die_again(),
            SECOND_VERDICT_FRAME => self.check_over_again(),
            _ => Step::Wait,
        }
    }
}

const KILL_FRAME: i32 = 148;
const PRESS_FRAME: i32 = 150;
const VERDICT_FRAME: i32 = 175;
const DIE_AGAIN_FRAME: i32 = 180;
const SECOND_VERDICT_FRAME: i32 = 190;

impl Restarting {
    fn kill(&mut self) -> Step {
        let Some(jumper) = harness::arena(|arena| arena.jumper) else {
            return fail("no arena");
        };
        self.jumper = jumper;
        destroy(jumper);
        Step::Wait
    }

    fn press(&mut self) -> Step {
        if mission::over() != Some(false) {
            return fail("losing the player unit did not end the game");
        }
        harness::press_restart();
        Step::Wait
    }

    fn check(&mut self) -> Step {
        if units::valid(self.jumper) {
            return fail("restart did not clear the old level");
        }
        let Some(units) = harness::arena(|arena| arena.units()) else {
            return fail("no arena after the restart");
        };
        let missing = units
            .iter()
            .filter(|unit_id| !units::valid(**unit_id))
            .count();
        if missing != 0 {
            return Step::Fail(alloc::format!(
                "{missing} of {} units missing after the restart",
                units.len()
            ));
        }
        if mission::over().is_some() {
            return fail("the restart left the game in its over state");
        }
        Step::Wait
    }

    fn die_again(&mut self) -> Step {
        let Some(jumper) = harness::arena(|arena| arena.jumper) else {
            return fail("no arena after the restart");
        };
        destroy(jumper);
        Step::Wait
    }

    fn check_over_again(&mut self) -> Step {
        if mission::over() != Some(false) {
            return fail("dying after a restart did not end the game again");
        }
        Step::Pass
    }
}

fn destroy(unit_id: i32) {
    let _ = spring::destroy_unit(
        unit_id,
        spring::generated::owned::unit_control::DestroyUnitOptions {
            selfd: false,
            reclaimed: false,
            attacker_id: -1,
            recycle_id: false,
        },
    );
}
