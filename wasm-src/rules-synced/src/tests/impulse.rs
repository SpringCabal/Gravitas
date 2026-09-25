use alloc::vec::Vec;

use spring::types::Float3;

use crate::state::Global;
use crate::tests::harness::{self, Case, Rest, STILL, Step, fail};
use crate::units;

pub struct PushAndPull {
    target: i32,
    attacker: i32,
    pushed_from: f32,
    pulled_from: f32,
    wall_at: Float3,
}

impl PushAndPull {
    pub fn new() -> Self {
        Self {
            target: -1,
            attacker: -1,
            pushed_from: 0.0,
            pulled_from: 0.0,
            wall_at: Float3::default(),
        }
    }
}

impl Case for PushAndPull {
    fn name(&self) -> &'static str {
        "impulse"
    }

    fn step(&mut self, global: &Global, frame: i32) -> Step {
        match frame {
            0 => self.adopt(global),
            PUSH_FRAME => self.push(global),
            PULL_FRAME => self.pull(global),
            VERDICT_FRAME => self.check_pulled(),
            _ => self.keep_firing(global, frame),
        }
    }

    fn rest(&self) -> Vec<Rest> {
        let mut rest = Vec::new();
        rest.push(Rest::at(
            self.attacker,
            self.wall_at,
            STILL,
            "the pushing wall",
        ));
        // The pull decides where the crate ends up; it must stay there.
        rest.extend(Rest::here(self.target, STILL, "the pushed crate"));
        rest
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
    fn adopt(&mut self, global: &Global) -> Step {
        let Some((target, attacker)) = harness::arena(global, |arena| (arena.pushed, arena.pusher))
        else {
            return fail("no arena");
        };
        let Some(wall_at) = harness::position(attacker) else {
            return fail("no position for the wall");
        };
        self.target = target;
        self.attacker = attacker;
        self.wall_at = wall_at;
        Step::Wait
    }

    fn keep_firing(&mut self, global: &Global, frame: i32) -> Step {
        if frame > PUSH_FRAME && frame <= PUSH_FRAME + BURST_FRAMES {
            return self.strike(global, PUSH_WEAPON);
        }
        if frame > PULL_FRAME && frame <= PULL_FRAME + BURST_FRAMES {
            return self.strike(global, PULL_WEAPON);
        }
        Step::Wait
    }

    fn push(&mut self, global: &Global) -> Step {
        let Some(separation) = self.separation() else {
            return fail("lost the crate or the attacker");
        };
        self.pushed_from = separation;
        self.strike(global, PUSH_WEAPON)
    }

    fn pull(&mut self, global: &Global) -> Step {
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
        self.strike(global, PULL_WEAPON)
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

    fn strike(&self, global: &Global, weapon_name: &str) -> Step {
        let (Ok(weapon), Some(def_id)) = (
            spring::weapon_defs::get_weapon_def_id(weapon_name),
            units::def_of(self.target),
        ) else {
            return Step::Fail(alloc::format!("no weapon def for {weapon_name}"));
        };
        global
            .impulses
            .access_mut()
            .damaged(self.target, def_id, weapon, self.attacker);
        Step::Wait
    }

    fn separation(&self) -> Option<f32> {
        let target = harness::position(self.target)?;
        let attacker = harness::position(self.attacker)?;
        Some(harness::planar_distance(attacker, target))
    }
}
