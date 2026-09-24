use spring::types::Float3;

use crate::state::Global;
use crate::tests::harness::{self, Case, Step, fail};
use crate::tests::record;

pub struct Shocking {
    victim: i32,
    park: Float3,
    entry_paralyze: f32,
    exit_health: f32,
}

impl Shocking {
    pub fn new() -> Self {
        Self {
            victim: -1,
            park: Float3::default(),
            entry_paralyze: 0.0,
            exit_health: 0.0,
        }
    }
}

impl Case for Shocking {
    fn name(&self) -> &'static str {
        "electrafi"
    }

    fn step(&mut self, global: &Global, frame: i32) -> Step {
        match frame {
            0 => self.enter(global),
            VERDICT_FRAME => self.check(),
            LINGER_FRAME => self.check_lingering(global),
            _ => Step::Wait,
        }
    }
}

const VERDICT_FRAME: i32 = 45;
/// Within the 60 frames of sparks after leaving the field.
const LINGER_FRAME: i32 = 75;
const CLEAR_OF_FIELD: f32 = 200.0;

impl Shocking {
    fn enter(&mut self, global: &Global) -> Step {
        let Some((field, victim)) =
            harness::arena(global, |arena| (arena.electrafi, arena.shocked))
        else {
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
        // Clear of the field: the robot's own spot is inside its reach.
        harness::place(
            self.victim,
            Float3 {
                x: self.park.x + CLEAR_OF_FIELD,
                ..self.park
            },
        );
        if paralyze <= self.entry_paralyze {
            return fail("standing in an electric field did not stun the robot");
        }
        let Some(health) = harness::health(self.victim) else {
            return fail("no health reading");
        };
        self.exit_health = health;
        Step::Wait
    }

    /// A robot that left the field keeps sparking for a while, unhurt.
    fn check_lingering(&mut self, global: &Global) -> Step {
        if !record::sparked_unhurt(global, self.victim) {
            return fail("the robot stopped sparking as soon as it left the field");
        }
        match harness::health(self.victim) {
            Some(health) if health < self.exit_health => {
                fail("the field kept hurting the robot after it left")
            }
            Some(_) => Step::Pass,
            None => fail("lost the shocked unit"),
        }
    }
}
