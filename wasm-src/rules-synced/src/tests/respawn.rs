use spring::types::Float3;

use crate::state::Global;
use crate::tests::harness::{self, Case, Step, fail};
use crate::units;

pub struct Respawning {
    origin: Float3,
    victim: i32,
}

impl Respawning {
    pub fn new() -> Self {
        Self {
            origin: Float3::default(),
            victim: -1,
        }
    }
}

impl Case for Respawning {
    fn name(&self) -> &'static str {
        "respawn"
    }

    fn step(&mut self, _global: &Global, frame: i32) -> Step {
        match frame {
            KILL_FRAME => self.kill(),
            VERDICT_FRAME => self.check(),
            _ => Step::Wait,
        }
    }
}

const KILL_FRAME: i32 = 36;
const VERDICT_FRAME: i32 = 60;
const REBORN_RANGE: f32 = 40.0;

impl Respawning {
    fn kill(&mut self) -> Step {
        let Some(victim) = harness::arena(|arena| arena.revivable) else {
            return fail("no arena");
        };
        let Some(origin) = harness::position(victim) else {
            return fail("no position for the respawnable unit");
        };
        self.victim = victim;
        self.origin = origin;
        let _ = spring::destroy_unit(
            victim,
            spring::generated::owned::unit_control::DestroyUnitOptions {
                selfd: false,
                reclaimed: false,
                attacker_id: -1,
                recycle_id: false,
            },
        );
        Step::Wait
    }

    fn check(&mut self) -> Step {
        if units::valid(self.victim) {
            return fail("the destroyed unit is somehow still alive");
        }
        let reborn = units::units_in_cylinder(self.origin.x, self.origin.z, REBORN_RANGE)
            .into_iter()
            .any(|unit_id| units::def_of(unit_id) == crate::defs::get().id(ROBOT));
        if !reborn {
            return fail("a respawnable unit did not come back");
        }
        Step::Pass
    }
}

const ROBOT: &str = "projector";
