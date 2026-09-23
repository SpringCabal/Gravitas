use spring::types::Float3;

use crate::state::Global;
use crate::tests::harness::{self, Case, Step, fail};

pub struct Shocking {
    victim: i32,
    park: Float3,
    entry_paralyze: f32,
}

impl Shocking {
    pub fn new() -> Self {
        Self {
            victim: -1,
            park: Float3::default(),
            entry_paralyze: 0.0,
        }
    }
}

impl Case for Shocking {
    fn name(&self) -> &'static str {
        "electrafi"
    }

    fn step(&mut self, _global: &Global, frame: i32) -> Step {
        match frame {
            0 => self.enter(),
            VERDICT_FRAME => self.check(),
            _ => Step::Wait,
        }
    }
}

const VERDICT_FRAME: i32 = 45;

impl Shocking {
    fn enter(&mut self) -> Step {
        let Some((field, victim)) = harness::arena(|arena| (arena.electrafi, arena.shocked)) else {
            return fail("no arena");
        };
        let (Some(park), Some(centre), Some(paralyze)) = (
            harness::position(victim),
            harness::position(field),
            harness::paralyze(victim),
        ) else {
            return fail("no position or health reading");
        };
        self.victim = victim;
        self.park = park;
        self.entry_paralyze = paralyze;
        harness::halt(victim);
        harness::place(victim, centre);
        Step::Wait
    }

    fn check(&mut self) -> Step {
        let Some(paralyze) = harness::paralyze(self.victim) else {
            return fail("lost the shocked unit");
        };
        harness::place(self.victim, self.park);
        if paralyze <= self.entry_paralyze {
            return fail("standing in an electric field did not stun the robot");
        }
        Step::Pass
    }
}
