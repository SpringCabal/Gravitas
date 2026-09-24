use spring::types::Float3;

use crate::state::Global;
use crate::tests::harness::{self, Case, Step, fail};

pub struct Falling {
    dropped: i32,
    cushioned: i32,
    dropped_health: f32,
    cushioned_health: f32,
}

impl Falling {
    pub fn new() -> Self {
        Self {
            dropped: -1,
            cushioned: -1,
            dropped_health: 0.0,
            cushioned_health: 0.0,
        }
    }
}

impl Case for Falling {
    fn name(&self) -> &'static str {
        "falldamage"
    }

    fn step(&mut self, global: &Global, frame: i32) -> Step {
        match frame {
            DROP_FRAME => self.drop(global, frame),
            SLAM_FRAME => self.slam(),
            VERDICT_FRAME => self.check(),
            _ => Step::Wait,
        }
    }
}

const DROP_FRAME: i32 = 2;
const SLAM_FRAME: i32 = 40;
const VERDICT_FRAME: i32 = 100;
const IMMUNITY_FRAMES: i32 = 120;

impl Falling {
    fn drop(&mut self, global: &Global, frame: i32) -> Step {
        let Some((dropped, cushioned)) =
            harness::arena(global, |arena| (arena.dropped, arena.cushioned))
        else {
            return fail("no arena");
        };
        let (Some(dropped_health), Some(cushioned_health)) =
            (harness::health(dropped), harness::health(cushioned))
        else {
            return fail("no health readings");
        };
        self.dropped = dropped;
        self.cushioned = cushioned;
        self.dropped_health = dropped_health;
        self.cushioned_health = cushioned_health;
        global
            .fall_immunity
            .access_mut()
            .set_immunity(cushioned, frame + IMMUNITY_FRAMES);
        lift(dropped);
        lift(cushioned);
        Step::Wait
    }

    fn slam(&mut self) -> Step {
        slam(self.dropped);
        slam(self.cushioned);
        Step::Wait
    }

    fn check(&mut self) -> Step {
        let (Some(dropped_health), Some(cushioned_health)) = (
            harness::health(self.dropped),
            harness::health(self.cushioned),
        ) else {
            return fail("a dropped unit did not survive the fall");
        };
        if dropped_health >= self.dropped_health {
            return fail("falling did no damage");
        }
        if cushioned_health < self.cushioned_health {
            return Step::Fail(alloc::format!(
                "a landing-immune unit lost {} health",
                self.cushioned_health - cushioned_health
            ));
        }
        Step::Pass
    }
}

fn lift(unit_id: i32) {
    let _ = spring::add_unit_impulse(
        unit_id,
        Float3 {
            x: 0.0,
            y: LAUNCH_IMPULSE,
            z: 0.0,
        },
        0.0,
    );
}

fn slam(unit_id: i32) {
    let _ = spring::set_unit_velocity(
        unit_id,
        Float3 {
            x: 0.0,
            y: -SLAM_SPEED,
            z: 0.0,
        },
    );
}

const LAUNCH_IMPULSE: f32 = 12.0;
const SLAM_SPEED: f32 = 6.0;
