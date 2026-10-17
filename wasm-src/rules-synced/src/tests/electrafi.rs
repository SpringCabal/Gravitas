use alloc::vec::Vec;

use spring::Float3;

use crate::state::Global;
use crate::tests::arena::RIG_SPACING;
use crate::tests::harness::{self, Case, Rest, STILL, Step, fail};
use crate::tests::record;

pub struct Shocking {
    field: i32,
    field_at: Float3,
    victim: i32,
    park: Float3,
    entry_paralyze: f32,
    exit_health: f32,
}

impl Shocking {
    pub fn new() -> Self {
        Self {
            field: -1,
            field_at: Float3::default(),
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

    fn step(
        &mut self,
        spring: &spring::Spring<'_, spring::RulesSynced>,
        global: &Global,
        frame: i32,
    ) -> Step {
        match frame {
            0 => self.enter(spring, global),
            VERDICT_FRAME => self.check(spring),
            LINGER_FRAME => self.check_lingering(spring, global),
            _ => Step::Wait,
        }
    }

    fn rest(&self, _spring: &spring::Spring<'_, spring::RulesSynced>) -> Vec<Rest> {
        alloc::vec![
            Rest::at(self.field, self.field_at, STILL, "the electric field"),
            Rest::at(
                self.victim,
                self.clear(),
                RIG_SPACING * 0.5,
                "the robot out of the field"
            ),
        ]
    }
}

const VERDICT_FRAME: i32 = 45;
const LINGER_FRAME: i32 = 75;
const CLEAR_OF_FIELD: f32 = 200.0;

impl Shocking {
    fn enter(&mut self, spring: &spring::Spring<'_, spring::RulesSynced>, global: &Global) -> Step {
        let Some((field, victim)) =
            harness::arena(global, |arena| (arena.electrafi, arena.shocked))
        else {
            return fail("no arena");
        };
        let (Some(park), Some(centre), Some(paralyze)) = (
            harness::position(spring, victim),
            harness::position(spring, field),
            harness::paralyze(spring, victim),
        ) else {
            return fail("no position or health reading");
        };
        self.field = field;
        self.field_at = centre;
        self.victim = victim;
        self.park = park;
        self.entry_paralyze = paralyze;
        harness::halt(victim);
        harness::place(spring, victim, centre);
        Step::Wait
    }

    fn check(&mut self, spring: &spring::Spring<'_, spring::RulesSynced>) -> Step {
        let Some(paralyze) = harness::paralyze(spring, self.victim) else {
            return fail("lost the shocked unit");
        };
        harness::place(spring, self.victim, self.clear());
        if paralyze <= self.entry_paralyze {
            return fail("standing in an electric field did not stun the robot");
        }
        let Some(health) = harness::health(spring, self.victim) else {
            return fail("no health reading");
        };
        self.exit_health = health;
        Step::Wait
    }

    fn clear(&self) -> Float3 {
        Float3 {
            x: self.park.x + CLEAR_OF_FIELD,
            ..self.park
        }
    }

    fn check_lingering(
        &mut self,
        spring: &spring::Spring<'_, spring::RulesSynced>,
        global: &Global,
    ) -> Step {
        if !record::sparked_unhurt(global, self.victim) {
            return fail("the robot stopped sparking as soon as it left the field");
        }
        match harness::health(spring, self.victim) {
            Some(health) if health < self.exit_health => {
                fail("the field kept hurting the robot after it left")
            }
            Some(_) => Step::Pass,
            None => fail("lost the shocked unit"),
        }
    }
}
