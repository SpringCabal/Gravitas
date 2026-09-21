use crate::state;
use crate::tests::harness::{self, Case, Step, fail};
use crate::units;

pub struct PushAndPull {
    target: i32,
    attacker: i32,
    pushed_from: f32,
    pulled_from: f32,
}

impl PushAndPull {
    pub fn new() -> Self {
        Self {
            target: -1,
            attacker: -1,
            pushed_from: 0.0,
            pulled_from: 0.0,
        }
    }
}

impl Case for PushAndPull {
    fn name(&self) -> &'static str {
        "impulse"
    }

    fn step(&mut self, frame: i32) -> Step {
        match frame {
            0 => self.adopt(),
            PUSH_FRAME => self.push(),
            PULL_FRAME => self.pull(),
            VERDICT_FRAME => self.check_pulled(),
            _ => self.keep_firing(frame),
        }
    }
}

const PUSH_FRAME: i32 = 5;
const PULL_FRAME: i32 = 50;
const VERDICT_FRAME: i32 = 95;
const BURST_FRAMES: i32 = 15;
const MARGIN: f32 = 5.0;
const PUSH_WEAPON: &str = "gravity_pos";
const PULL_WEAPON: &str = "gravity_neg";

impl PushAndPull {
    fn adopt(&mut self) -> Step {
        let Some((target, attacker)) = harness::arena(|arena| (arena.pushed, arena.pusher)) else {
            return fail("no arena");
        };
        self.target = target;
        self.attacker = attacker;
        Step::Wait
    }

    fn keep_firing(&mut self, frame: i32) -> Step {
        if frame > PUSH_FRAME && frame <= PUSH_FRAME + BURST_FRAMES {
            return self.strike(PUSH_WEAPON);
        }
        if frame > PULL_FRAME && frame <= PULL_FRAME + BURST_FRAMES {
            return self.strike(PULL_WEAPON);
        }
        Step::Wait
    }

    fn push(&mut self) -> Step {
        let Some(separation) = self.separation() else {
            return fail("lost the crate or the attacker");
        };
        self.pushed_from = separation;
        self.strike(PUSH_WEAPON)
    }

    fn pull(&mut self) -> Step {
        let Some(separation) = self.separation() else {
            return fail("lost the crate or the attacker");
        };
        if separation <= self.pushed_from + MARGIN {
            return Step::Fail(alloc::format!(
                "push moved the crate from {} to {separation}",
                self.pushed_from
            ));
        }
        self.pulled_from = separation;
        self.strike(PULL_WEAPON)
    }

    fn check_pulled(&mut self) -> Step {
        let Some(separation) = self.separation() else {
            return fail("lost the crate or the attacker");
        };
        if separation >= self.pulled_from - MARGIN {
            return Step::Fail(alloc::format!(
                "pull moved the crate from {} to {separation}",
                self.pulled_from
            ));
        }
        Step::Pass
    }

    fn strike(&self, weapon_name: &str) -> Step {
        let (Ok(weapon), Some(def_id)) = (
            spring::weapon_defs::get_weapon_def_id(weapon_name),
            units::def_of(self.target),
        ) else {
            return Step::Fail(alloc::format!("no weapon def for {weapon_name}"));
        };
        let applied = state::with_world(|world| {
            world
                .impulses
                .damaged(self.target, def_id, weapon, self.attacker)
        });
        if applied.is_none() {
            return fail("world unavailable");
        }
        Step::Wait
    }

    fn separation(&self) -> Option<f32> {
        let target = harness::position(self.target)?;
        let attacker = harness::position(self.attacker)?;
        Some(harness::planar_distance(attacker, target))
    }
}
