use alloc::vec::Vec;

use spring::Float3;

use crate::state::Global;
use crate::tests::arena::RIG_SPACING;
use crate::tests::harness::{self, Case, Rest, STILL, Step, fail};

pub struct Collision {
    thrown: i32,
    anvil: i32,
    origin: Float3,
    health: f32,
}

impl Collision {
    pub fn new() -> Self {
        Self {
            thrown: -1,
            anvil: -1,
            origin: Float3::default(),
            health: 0.0,
        }
    }
}

impl Case for Collision {
    fn name(&self) -> &'static str {
        "collision"
    }

    fn step(
        &mut self,
        spring: &spring::Spring<'_, spring::RulesSynced>,
        global: &Global,
        frame: i32,
    ) -> Step {
        match frame {
            THROW_FRAME => self.throw(spring, global, frame),
            VERDICT_FRAME => self.check(spring),
            _ => Step::Wait,
        }
    }

    fn rest(&self, spring: &spring::Spring<'_, spring::RulesSynced>) -> Vec<Rest> {
        let mut rest = alloc::vec![Rest::at(
            self.thrown,
            self.origin,
            RIG_SPACING * 0.5,
            "the thrown robot"
        )];
        rest.extend(Rest::here(
            spring,
            self.anvil,
            STILL,
            "the crate the robot hit",
        ));
        rest
    }
}

const THROW_FRAME: i32 = 2;
const VERDICT_FRAME: i32 = 60;
const IMMUNITY_FRAMES: i32 = 120;
const THROW: Float3 = Float3 {
    x: -8.0,
    y: 3.0,
    z: 0.0,
};

impl Collision {
    fn throw(
        &mut self,
        spring: &spring::Spring<'_, spring::RulesSynced>,
        global: &Global,
        frame: i32,
    ) -> Step {
        let Some((thrown, anvil)) = harness::arena(global, |arena| (arena.thrown, arena.anvil))
        else {
            return fail("no arena");
        };
        let (Some(health), Some(origin)) = (
            harness::health(spring, thrown),
            harness::position(spring, thrown),
        ) else {
            return fail("no health or position reading");
        };
        self.thrown = thrown;
        self.anvil = anvil;
        self.origin = origin;
        self.health = health;
        global
            .fall_immunity
            .access_mut()
            .set_immunity(thrown, frame + IMMUNITY_FRAMES);
        if spring
            .unit_control()
            .add_unit_impulse(thrown, THROW, 0.0)
            .is_err()
        {
            return fail("the throw was refused");
        }
        Step::Wait
    }

    fn check(&mut self, spring: &spring::Spring<'_, spring::RulesSynced>) -> Step {
        match harness::health(spring, self.thrown) {
            Some(health) if health >= self.health => fail("hitting the crate did no damage"),
            _ => Step::Pass,
        }
    }
}
