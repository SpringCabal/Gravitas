use alloc::vec::Vec;

use spring::types::Float3;

use crate::state::Global;
use crate::tests::harness::{self, Case, Rest, Step, fail};
use crate::units;

/// A Gravit that dies plays gravit.lua's death animation before it goes, and
/// a robot throws its parts off in three stages (projector.lua).
pub struct Dying {
    unit_id: i32,
    head: Float3,
    robot: i32,
}

impl Dying {
    pub fn new() -> Self {
        Self {
            unit_id: -1,
            head: Float3::default(),
            robot: -1,
        }
    }
}

impl Case for Dying {
    fn name(&self) -> &'static str {
        "death"
    }

    fn step(&mut self, global: &Global, frame: i32) -> Step {
        match frame {
            KILL_FRAME => self.kill(global),
            ANIMATING_FRAME => self.check_animating(),
            GONE_FRAME => self.check_gone(),
            _ => Step::Wait,
        }
    }

    fn rest(&self) -> Vec<Rest> {
        alloc::vec![
            Rest::Gone {
                unit: self.unit_id,
                what: "the dead Gravit",
            },
            Rest::Gone {
                unit: self.robot,
                what: "the dead robot",
            },
        ]
    }
}

const KILL_FRAME: i32 = 2;
const ANIMATING_FRAME: i32 = KILL_FRAME + 10;
/// The animation is 25 frames long.
const GONE_FRAME: i32 = KILL_FRAME + 33;
const HEAD: &str = "head";
const MOVED: f32 = 0.5;

impl Dying {
    fn kill(&mut self, global: &Global) -> Step {
        let Some((unit_id, robot)) =
            harness::arena(global, |arena| (arena.dying_gravit, arena.dying_robot))
        else {
            return fail("no arena");
        };
        let Some(head) = harness::piece_position(unit_id, HEAD) else {
            return fail("no head piece");
        };
        self.unit_id = unit_id;
        self.head = head;
        self.robot = robot;
        harness::destroy(unit_id);
        harness::destroy(robot);
        Step::Wait
    }

    fn check_animating(&mut self) -> Step {
        if !units::valid(self.unit_id) {
            return fail("the Gravit was removed without its death animation");
        }
        let head = harness::piece_position(self.unit_id, HEAD);
        if !harness::moved(Some(self.head), head, MOVED) {
            return fail("the dying Gravit's head did not move");
        }
        // 10 frames in: past the second of the robot's three stages (6 frames
        // apart), not yet at the last.
        if !units::valid(self.robot) {
            return fail("the robot was removed before its last death stage");
        }
        Step::Wait
    }

    fn check_gone(&mut self) -> Step {
        if units::valid(self.unit_id) {
            return fail("the Gravit was still there after its death animation");
        }
        if units::valid(self.robot) {
            return fail("the robot was still there after its death stages");
        }
        Step::Pass
    }
}
