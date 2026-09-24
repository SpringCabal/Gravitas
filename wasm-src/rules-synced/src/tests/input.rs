use spring::types::Float3;

use crate::state::Global;
use crate::tests::harness::{self, Case, Step, fail};

pub struct Input {
    unit_id: i32,
    origin: Float3,
}

impl Input {
    pub fn new() -> Self {
        Self {
            unit_id: -1,
            origin: Float3::default(),
        }
    }
}

impl Case for Input {
    fn name(&self) -> &'static str {
        "input"
    }

    fn step(&mut self, global: &Global, frame: i32) -> Step {
        match frame {
            PRESS_FRAME => self.press(global),
            VERDICT_FRAME => self.check(),
            _ => Step::Wait,
        }
    }
}

const PRESS_FRAME: i32 = 4;
const VERDICT_FRAME: i32 = 120;
const REACH: f32 = 300.0;
const TOLERANCE: f32 = 60.0;

impl Input {
    fn press(&mut self, global: &Global) -> Step {
        let Some(unit_id) = harness::arena(global, |arena| arena.jumper) else {
            return fail("no arena");
        };
        let Some(origin) = harness::position(unit_id) else {
            return fail("no player position");
        };
        self.unit_id = unit_id;
        self.origin = origin;
        harness::press_jump(origin.x + REACH, origin.z);
        Step::Wait
    }

    fn check(&mut self) -> Step {
        harness::release_pointer();
        let Some(position) = harness::position(self.unit_id) else {
            return fail("no player position");
        };
        let travelled = harness::planar_distance(self.origin, position);
        if travelled < REACH - TOLERANCE {
            return Step::Fail(alloc::format!(
                "pressing jump moved the player {travelled} of {REACH}"
            ));
        }
        Step::Pass
    }
}
