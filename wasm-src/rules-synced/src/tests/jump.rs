use alloc::vec::Vec;

use spring::Float3;

use crate::state::Global;
use crate::tests::harness::{self, Case, Rest, Step, fail};

pub struct Jumping {
    unit_id: i32,
    origin: Float3,
    goal: Float3,
}

impl Jumping {
    pub fn new() -> Self {
        Self {
            unit_id: -1,
            origin: Float3::default(),
            goal: Float3::default(),
        }
    }
}

impl Case for Jumping {
    fn name(&self) -> &'static str {
        "jump"
    }

    fn step(
        &mut self,
        spring: &spring::Spring<'_, spring::RulesSynced>,
        global: &Global,
        frame: i32,
    ) -> Step {
        match frame {
            LEAP_FRAME => self.leap(spring, global),
            AIRBORNE_FRAME => self.check_airborne(spring, global),
            VERDICT_FRAME => self.check_landed(spring, global),
            _ => Step::Wait,
        }
    }

    fn rest(&self, _spring: &spring::Spring<'_, spring::RulesSynced>) -> Vec<Rest> {
        alloc::vec![Rest::at(
            self.unit_id,
            self.goal,
            TOLERANCE,
            "the Gravit that jumped"
        )]
    }
}

const LEAP_FRAME: i32 = 2;
const AIRBORNE_FRAME: i32 = 12;
const VERDICT_FRAME: i32 = 130;
const REACH: f32 = 300.0;
const OUT_OF_REACH: f32 = 2000.0;
const TOLERANCE: f32 = 60.0;
const LEDGE: f32 = 200.0;
const DIRECTIONS: [(f32, f32); 4] = [(1.0, 0.0), (-1.0, 0.0), (0.0, 1.0), (0.0, -1.0)];

impl Jumping {
    fn leap(&mut self, spring: &spring::Spring<'_, spring::RulesSynced>, global: &Global) -> Step {
        let Some(unit_id) = harness::arena(global, |arena| arena.vaulter) else {
            return fail("no arena");
        };
        let Some(origin) = harness::position(spring, unit_id) else {
            return fail("no jumper position");
        };
        self.unit_id = unit_id;
        self.origin = origin;
        set_speed(spring, unit_id, 0.0);
        let still = request(spring, global, unit_id, origin.x + REACH, origin.z);
        set_speed(spring, unit_id, crate::gadgets::jump::behavior::SPEED);
        if still {
            return fail("a jump with jumpSpeed 0 was accepted");
        }
        if request(spring, global, unit_id, -1000.0, origin.z) {
            return fail("a jump outside the map was accepted");
        }
        if request_at(
            spring,
            global,
            unit_id,
            origin.x,
            origin.y + LEDGE,
            origin.z,
        ) {
            return fail("a jump onto a ledge was accepted");
        }
        for (x, z) in DIRECTIONS {
            let goal = Float3 {
                x: origin.x + x * REACH,
                z: origin.z + z * REACH,
                ..origin
            };
            if request(spring, global, unit_id, goal.x, goal.z) {
                self.goal = goal;
                return Step::Wait;
            }
        }
        Step::Fail(alloc::format!("no {REACH} elmo jump was accepted"))
    }

    fn check_airborne(
        &mut self,
        spring: &spring::Spring<'_, spring::RulesSynced>,
        global: &Global,
    ) -> Step {
        let jumping = global.jumps.access().is_jumping(self.unit_id);
        if !jumping {
            return fail("the unit was not airborne");
        }
        if request(spring, global, self.unit_id, self.origin.x, self.origin.z) {
            return fail("a second jump started while airborne");
        }
        Step::Wait
    }

    fn check_landed(
        &mut self,
        spring: &spring::Spring<'_, spring::RulesSynced>,
        global: &Global,
    ) -> Step {
        let Some(position) = harness::position(spring, self.unit_id) else {
            return fail("no jumper position");
        };
        let travelled = harness::planar_distance(self.origin, position);
        if travelled < REACH - TOLERANCE {
            return Step::Fail(alloc::format!("jump travelled {travelled} of {REACH}"));
        }
        if request(
            spring,
            global,
            self.unit_id,
            position.x + OUT_OF_REACH,
            position.z,
        ) {
            return fail("a jump beyond range was accepted");
        }
        harness::halt(self.unit_id);
        Step::Pass
    }
}

fn request(
    spring: &spring::Spring<'_, spring::RulesSynced>,
    global: &Global,
    unit_id: i32,
    x: f32,
    z: f32,
) -> bool {
    request_at(
        spring,
        global,
        unit_id,
        x,
        crate::units::ground_height(spring, x, z),
        z,
    )
}

fn request_at(
    spring: &spring::Spring<'_, spring::RulesSynced>,
    global: &Global,
    unit_id: i32,
    x: f32,
    y: f32,
    z: f32,
) -> bool {
    crate::gadgets::jump::behavior::start(spring, global, unit_id, x, y, z)
        == crate::gadgets::jump::behavior::Outcome::Started
}

fn set_speed(spring: &spring::Spring<'_, spring::RulesSynced>, unit_id: i32, speed: f32) {
    let _ = spring.rules_params().set_unit_rules_param_f32(
        unit_id,
        SPEED_PARAM,
        speed,
        crate::units::RULES_PARAM_PRIVATE,
    );
}

const SPEED_PARAM: &str = "jumpSpeed";
