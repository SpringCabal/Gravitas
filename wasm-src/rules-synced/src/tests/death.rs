use alloc::vec::Vec;

use spring::Float3;

use crate::state::Global;
use crate::tests::harness::{self, Case, Rest, Step, fail};
use crate::units;

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

    fn step(
        &mut self,
        spring: &spring::Spring<'_, spring::RulesSynced>,
        global: &Global,
        frame: i32,
    ) -> Step {
        match frame {
            KILL_FRAME => self.kill(spring, global),
            ANIMATING_FRAME => self.check_animating(spring),
            GONE_FRAME => self.check_gone(spring),
            _ => Step::Wait,
        }
    }

    fn rest(&self, _spring: &spring::Spring<'_, spring::RulesSynced>) -> Vec<Rest> {
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
const GONE_FRAME: i32 = KILL_FRAME + 33;
const HEAD: &str = "head";
const MOVED: f32 = 0.5;

impl Dying {
    fn kill(&mut self, spring: &spring::Spring<'_, spring::RulesSynced>, global: &Global) -> Step {
        let Some((unit_id, robot)) =
            harness::arena(global, |arena| (arena.dying_gravit, arena.dying_robot))
        else {
            return fail("no arena");
        };
        let Some(head) = harness::piece_position(spring, unit_id, HEAD) else {
            return fail("no head piece");
        };
        self.unit_id = unit_id;
        self.head = head;
        self.robot = robot;
        harness::destroy(unit_id);
        harness::destroy(robot);
        Step::Wait
    }

    fn check_animating(&mut self, spring: &spring::Spring<'_, spring::RulesSynced>) -> Step {
        if !units::valid(spring, self.unit_id) {
            return fail("the Gravit was removed without its death animation");
        }
        let head = harness::piece_position(spring, self.unit_id, HEAD);
        if !harness::moved(Some(self.head), head, MOVED) {
            return fail("the dying Gravit's head did not move");
        }
        if !units::valid(spring, self.robot) {
            return fail("the robot was removed before its last death stage");
        }
        Step::Wait
    }

    fn check_gone(&mut self, spring: &spring::Spring<'_, spring::RulesSynced>) -> Step {
        if units::valid(spring, self.unit_id) {
            return fail("the Gravit was still there after its death animation");
        }
        if units::valid(spring, self.robot) {
            return fail("the robot was still there after its death stages");
        }
        Step::Pass
    }
}
