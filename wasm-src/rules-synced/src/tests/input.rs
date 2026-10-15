use alloc::vec::Vec;

use spring::types::Float3;

use crate::state::Global;
use crate::tests::harness::{self, Case, Rest, Step, fail};

pub struct Input {
    unit_id: i32,
    origin: Float3,
    goal: Float3,
    stage: Stage,
    second_jump: bool,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Stage {
    Press,
    TakeOff,
    Land,
}

impl Input {
    pub fn new() -> Self {
        Self {
            unit_id: -1,
            origin: Float3::default(),
            goal: Float3::default(),
            stage: Stage::Press,
            second_jump: false,
        }
    }
}

impl Case for Input {
    fn name(&self) -> &'static str {
        "input"
    }

    fn step(&mut self, global: &Global, _frame: i32) -> Step {
        let jumping = global.jumps.access().is_jumping(self.unit_id);
        match self.stage {
            Stage::Press => self.press(global),
            Stage::TakeOff if jumping => {
                harness::release_pointer();
                self.stage = Stage::Land;
                Step::Wait
            }
            Stage::Land if !jumping => self.check(),
            _ => Step::Wait,
        }
    }

    fn rest(&self) -> Vec<Rest> {
        alloc::vec![Rest::at(
            self.unit_id,
            self.goal,
            TOLERANCE,
            "the player's Gravit"
        )]
    }
}

const REACH: f32 = 150.0;
const TOLERANCE: f32 = 60.0;

impl Input {
    fn press(&mut self, global: &Global) -> Step {
        let Some(unit_id) = harness::arena(global, |arena| arena.jumper) else {
            return fail("no arena");
        };
        if global.player_unit.get() != Some(unit_id) {
            return fail("the player does not drive the arena's last Gravit");
        }
        let Some(origin) = harness::position(unit_id) else {
            return fail("no player position");
        };
        self.unit_id = unit_id;
        self.origin = origin;
        self.goal = Float3 {
            z: origin.z - REACH,
            ..origin
        };
        harness::press_jump(self.goal.x, self.goal.z);
        self.stage = Stage::TakeOff;
        Step::Wait
    }

    fn check(&mut self) -> Step {
        let Some(position) = harness::position(self.unit_id) else {
            return fail("no player position");
        };
        let travelled = harness::planar_distance(self.origin, position);
        if travelled < REACH - TOLERANCE {
            return Step::Fail(alloc::format!(
                "pressing jump moved the player {travelled} of {REACH}"
            ));
        }
        if self.second_jump {
            return Step::Pass;
        }
        self.second_jump = true;
        self.origin = position;
        self.goal.z -= REACH;
        harness::press_jump(self.goal.x, self.goal.z);
        self.stage = Stage::TakeOff;
        Step::Wait
    }
}
