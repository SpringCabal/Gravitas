use crate::state::Global;
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

    fn step(&mut self, global: &Global, frame: i32) -> Step {
        match frame {
            OTHER_GRAVIT_FRAME => self.kill_other_gravit(global),
            KILL_FRAME => self.kill(global),
            PRESS_FRAME => self.press(global),
            VERDICT_FRAME => self.check(global),
            DIE_AGAIN_FRAME => self.die_again(global),
            SECOND_VERDICT_FRAME => self.check_over_again(global),
            _ => Step::Wait,
        }
    }
}

/// After the jump case is done with its Gravit.
const OTHER_GRAVIT_FRAME: i32 = 144;
const KILL_FRAME: i32 = 148;
const PRESS_FRAME: i32 = 150;
/// After the killed player unit's death animation (25 frames).
const VERDICT_FRAME: i32 = 180;
const DIE_AGAIN_FRAME: i32 = 185;
const SECOND_VERDICT_FRAME: i32 = 195;

impl Restarting {
    fn kill_other_gravit(&mut self, global: &Global) -> Step {
        let Some(vaulter) = harness::arena(global, |arena| arena.vaulter) else {
            return fail("no arena");
        };
        destroy(vaulter);
        if global.outcome.get().is_some() {
            return fail("losing a Gravit that is not the player's ended the game");
        }
        Step::Wait
    }

    fn kill(&mut self, global: &Global) -> Step {
        let Some(jumper) = harness::arena(global, |arena| arena.jumper) else {
            return fail("no arena");
        };
        self.jumper = jumper;
        destroy(jumper);
        Step::Wait
    }

    fn press(&mut self, global: &Global) -> Step {
        if global.outcome.get() != Some(false) {
            return fail("losing the player unit did not end the game");
        }
        harness::press_restart(global);
        Step::Wait
    }

    fn check(&mut self, global: &Global) -> Step {
        if units::valid(self.jumper) {
            return fail("restart did not clear the old level");
        }
        let Some(units) = harness::arena(global, |arena| arena.units()) else {
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
        if global.outcome.get().is_some() {
            return fail("the restart left the game in its over state");
        }
        Step::Wait
    }

    fn die_again(&mut self, global: &Global) -> Step {
        let Some(jumper) = harness::arena(global, |arena| arena.jumper) else {
            return fail("no arena after the restart");
        };
        destroy(jumper);
        Step::Wait
    }

    fn check_over_again(&mut self, global: &Global) -> Step {
        if global.outcome.get() != Some(false) {
            return fail("dying after a restart did not end the game again");
        }
        Step::Pass
    }
}

pub fn destroy(unit_id: i32) {
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
