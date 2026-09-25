use alloc::vec::Vec;

use spring::types::Float3;

use crate::state::Global;
use crate::tests::arena::RIG_SPACING;
use crate::tests::harness::{self, Case, Rest, STILL, Step, fail};

/// A robot thrown into a crate takes the engine's collision damage. The robot
/// is immune to fall damage for the throw, so a hard landing can't pass for
/// the collision.
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

    fn step(&mut self, global: &Global, frame: i32) -> Step {
        match frame {
            THROW_FRAME => self.throw(global, frame),
            VERDICT_FRAME => self.check(),
            _ => Step::Wait,
        }
    }

    /// The bounce decides where the robot and the crate stop; they stay in
    /// the rig.
    fn rest(&self) -> Vec<Rest> {
        let mut rest = alloc::vec![Rest::at(
            self.thrown,
            self.origin,
            RIG_SPACING * 0.5,
            "the thrown robot"
        )];
        rest.extend(Rest::here(self.anvil, STILL, "the crate the robot hit"));
        rest
    }
}

const THROW_FRAME: i32 = 2;
const VERDICT_FRAME: i32 = 60;
const IMMUNITY_FRAMES: i32 = 120;
/// Towards the crate, and up enough that the robot leaves the ground.
const THROW: Float3 = Float3 {
    x: -8.0,
    y: 3.0,
    z: 0.0,
};

impl Collision {
    fn throw(&mut self, global: &Global, frame: i32) -> Step {
        let Some((thrown, anvil)) = harness::arena(global, |arena| (arena.thrown, arena.anvil))
        else {
            return fail("no arena");
        };
        let (Some(health), Some(origin)) = (harness::health(thrown), harness::position(thrown))
        else {
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
        if spring::add_unit_impulse(thrown, THROW, 0.0).is_err() {
            return fail("the throw was refused");
        }
        Step::Wait
    }

    fn check(&mut self) -> Step {
        match harness::health(self.thrown) {
            Some(health) if health >= self.health => fail("hitting the crate did no damage"),
            _ => Step::Pass,
        }
    }
}
