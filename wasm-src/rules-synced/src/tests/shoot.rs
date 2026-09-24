use spring::types::Float3;

use crate::state::Global;
use crate::tests::harness::{self, Case, Step, fail};
use crate::units;

/// A robot shoots while it walks: the Lua AimWeapon functions never refuse a
/// shot for moving.
pub struct Shooting {
    gunner: i32,
    target: i32,
    origin: Float3,
    target_health: f32,
}

impl Shooting {
    pub fn new() -> Self {
        Self {
            gunner: -1,
            target: -1,
            origin: Float3::default(),
            target_health: 0.0,
        }
    }
}

impl Case for Shooting {
    fn name(&self) -> &'static str {
        "shoot"
    }

    fn step(&mut self, global: &Global, frame: i32) -> Step {
        match frame {
            WALK_FRAME => self.walk(global),
            VERDICT_FRAME => self.check(),
            _ => Step::Wait,
        }
    }
}

const WALK_FRAME: i32 = 2;
const VERDICT_FRAME: i32 = 60;
/// Away from the target, and far enough to still be walking at the verdict.
const WALK_DISTANCE: f32 = 250.0;
const FIRE_AT_WILL: f32 = 2.0;
const MOVED: f32 = 20.0;

impl Shooting {
    fn walk(&mut self, global: &Global) -> Step {
        let Some((gunner, target)) = harness::arena(global, |arena| (arena.gunner, arena.target))
        else {
            return fail("no arena");
        };
        let (Some(origin), Some(health)) = (harness::position(gunner), harness::health(target))
        else {
            return fail("no position or health reading");
        };
        self.gunner = gunner;
        self.target = target;
        self.origin = origin;
        self.target_health = health;
        units::order(gunner, spring::cmd::FIRE_STATE, &[FIRE_AT_WILL]);
        let z = origin.z + WALK_DISTANCE;
        units::order(
            gunner,
            spring::cmd::MOVE,
            &[origin.x, units::ground_height(origin.x, z), z],
        );
        Step::Wait
    }

    fn check(&mut self) -> Step {
        let still_walking = spring::get_unit_commands(self.gunner, 1)
            .is_ok_and(|queue| queue.first().is_some_and(|c| c.cmd_id == spring::cmd::MOVE));
        if !harness::moved(Some(self.origin), harness::position(self.gunner), MOVED)
            || !still_walking
        {
            return fail("the robot was not walking at the verdict");
        }
        let health = harness::health(self.target).unwrap_or(0.0);
        if health >= self.target_health {
            return fail("the walking robot never hit its target");
        }
        Step::Pass
    }
}
