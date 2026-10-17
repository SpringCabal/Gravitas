use alloc::vec::Vec;

use spring::Float3;

use crate::state::Global;
use crate::tests::harness::{self, Case, Rest, STILL, Step, fail};

pub struct Falling {
    dropped: i32,
    cushioned: i32,
    dropped_health: f32,
    cushioned_health: f32,
    dropped_at: Float3,
    cushioned_at: Float3,
}

impl Falling {
    pub fn new() -> Self {
        Self {
            dropped: -1,
            cushioned: -1,
            dropped_health: 0.0,
            cushioned_health: 0.0,
            dropped_at: Float3::default(),
            cushioned_at: Float3::default(),
        }
    }
}

impl Case for Falling {
    fn name(&self) -> &'static str {
        "falldamage"
    }

    fn step(
        &mut self,
        spring: &spring::Spring<'_, spring::RulesSynced>,
        global: &Global,
        frame: i32,
    ) -> Step {
        match frame {
            DROP_FRAME => self.drop(spring, global, frame),
            SLAM_FRAME => self.slam(spring),
            VERDICT_FRAME => self.check(spring),
            _ => Step::Wait,
        }
    }

    fn rest(&self, _spring: &spring::Spring<'_, spring::RulesSynced>) -> Vec<Rest> {
        alloc::vec![
            Rest::at(self.dropped, self.dropped_at, STILL, "the dropped robot"),
            Rest::at(
                self.cushioned,
                self.cushioned_at,
                STILL,
                "the cushioned robot"
            ),
        ]
    }
}

const DROP_FRAME: i32 = 2;
const SLAM_FRAME: i32 = 40;
const VERDICT_FRAME: i32 = 100;
const IMMUNITY_FRAMES: i32 = 120;

impl Falling {
    fn drop(
        &mut self,
        spring: &spring::Spring<'_, spring::RulesSynced>,
        global: &Global,
        frame: i32,
    ) -> Step {
        let Some((dropped, cushioned)) =
            harness::arena(global, |arena| (arena.dropped, arena.cushioned))
        else {
            return fail("no arena");
        };
        let (Some(dropped_health), Some(cushioned_health)) = (
            harness::health(spring, dropped),
            harness::health(spring, cushioned),
        ) else {
            return fail("no health readings");
        };
        let (Some(dropped_at), Some(cushioned_at)) = (
            harness::position(spring, dropped),
            harness::position(spring, cushioned),
        ) else {
            return fail("no positions");
        };
        self.dropped_at = dropped_at;
        self.cushioned_at = cushioned_at;
        self.dropped = dropped;
        self.cushioned = cushioned;
        self.dropped_health = dropped_health;
        self.cushioned_health = cushioned_health;
        global
            .fall_immunity
            .access_mut()
            .set_immunity(cushioned, frame + IMMUNITY_FRAMES);
        lift(spring, dropped);
        lift(spring, cushioned);
        Step::Wait
    }

    fn slam(&mut self, spring: &spring::Spring<'_, spring::RulesSynced>) -> Step {
        slam(spring, self.dropped);
        slam(spring, self.cushioned);
        Step::Wait
    }

    fn check(&mut self, spring: &spring::Spring<'_, spring::RulesSynced>) -> Step {
        let (Some(dropped_health), Some(cushioned_health)) = (
            harness::health(spring, self.dropped),
            harness::health(spring, self.cushioned),
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

fn lift(spring: &spring::Spring<'_, spring::RulesSynced>, unit_id: i32) {
    let _ = spring.unit_control().add_unit_impulse(
        unit_id,
        Float3 {
            x: 0.0,
            y: LAUNCH_IMPULSE,
            z: 0.0,
        },
        0.0,
    );
}

fn slam(spring: &spring::Spring<'_, spring::RulesSynced>, unit_id: i32) {
    let _ = spring.unit_control().set_unit_velocity(
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
