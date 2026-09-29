use alloc::vec::Vec;

use spring::types::Float3;

use crate::state::Global;
use crate::tests::harness::{self, Case, Rest, Step, fail};
use crate::units;

pub struct Respawning {
    origin: Float3,
    victim: i32,
    reborn: i32,
}

impl Respawning {
    pub fn new() -> Self {
        Self {
            origin: Float3::default(),
            victim: -1,
            reborn: -1,
        }
    }
}

impl Case for Respawning {
    fn name(&self) -> &'static str {
        "respawn"
    }

    fn step(&mut self, global: &Global, frame: i32) -> Step {
        match frame {
            KILL_FRAME => self.kill(global),
            VERDICT_FRAME => self.check(),
            _ => Step::Wait,
        }
    }

    fn rest(&self) -> Vec<Rest> {
        alloc::vec![Rest::at(
            self.reborn,
            self.origin,
            REBORN_RANGE,
            "the respawned robot"
        )]
    }
}

const KILL_FRAME: i32 = 36;
const VERDICT_FRAME: i32 = 60;
const REBORN_RANGE: f32 = 40.0;

impl Respawning {
    fn kill(&mut self, global: &Global) -> Step {
        let Some(victim) = harness::arena(global, |arena| arena.revivable) else {
            return fail("no arena");
        };
        let Some(origin) = harness::position(victim) else {
            return fail("no position for the respawnable unit");
        };
        self.victim = victim;
        self.origin = origin;
        harness::destroy(victim);
        Step::Wait
    }

    fn check(&mut self) -> Step {
        if units::valid(self.victim) {
            return fail("the destroyed unit is somehow still alive");
        }
        let reborn = units::units_in_cylinder(self.origin.x, self.origin.z, REBORN_RANGE)
            .into_iter()
            .find(|unit_id| units::def_of(*unit_id) == crate::defs::get().id(ROBOT));
        let Some(reborn) = reborn else {
            return fail("a respawnable unit did not come back");
        };
        if !crate::effects::is_respawnable(reborn) {
            return fail("the respawned unit is not respawnable");
        }
        self.reborn = reborn;
        Step::Pass
    }
}

const ROBOT: &str = "projector";
