use spring::types::Float3;

use crate::state::Global;
use crate::tests::harness::{self, Case, Step, fail};

pub struct Jumping {
    unit_id: i32,
    origin: Float3,
}

impl Jumping {
    pub fn new() -> Self {
        Self {
            unit_id: -1,
            origin: Float3::default(),
        }
    }
}

impl Case for Jumping {
    fn name(&self) -> &'static str {
        "jump"
    }

    fn step(&mut self, global: &Global, frame: i32) -> Step {
        match frame {
            LEAP_FRAME => self.leap(global),
            AIRBORNE_FRAME => self.check_airborne(global),
            VERDICT_FRAME => self.check_landed(global),
            _ => Step::Wait,
        }
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
    fn leap(&mut self, global: &Global) -> Step {
        let Some(unit_id) = harness::arena(global, |arena| arena.vaulter) else {
            return fail("no arena");
        };
        let Some(origin) = harness::position(unit_id) else {
            return fail("no jumper position");
        };
        self.unit_id = unit_id;
        self.origin = origin;
        if request(global, unit_id, -1000.0, origin.z) {
            return fail("a jump outside the map was accepted");
        }
        if request_at(global, unit_id, origin.x, origin.y + LEDGE, origin.z) {
            return fail("a jump onto a ledge was accepted");
        }
        for (x, z) in DIRECTIONS {
            if request(global, unit_id, origin.x + x * REACH, origin.z + z * REACH) {
                return Step::Wait;
            }
        }
        Step::Fail(alloc::format!("no {REACH} elmo jump was accepted"))
    }

    fn check_airborne(&mut self, global: &Global) -> Step {
        let jumping = global.jumps.access().is_jumping(self.unit_id);
        if !jumping {
            return fail("the unit was not airborne");
        }
        if request(global, self.unit_id, self.origin.x, self.origin.z) {
            return fail("a second jump started while airborne");
        }
        Step::Wait
    }

    fn check_landed(&mut self, global: &Global) -> Step {
        let Some(position) = harness::position(self.unit_id) else {
            return fail("no jumper position");
        };
        let travelled = harness::planar_distance(self.origin, position);
        if travelled < REACH - TOLERANCE {
            return Step::Fail(alloc::format!("jump travelled {travelled} of {REACH}"));
        }
        if request(global, self.unit_id, position.x + OUT_OF_REACH, position.z) {
            return fail("a jump beyond range was accepted");
        }
        harness::halt(self.unit_id);
        Step::Pass
    }
}

fn request(global: &Global, unit_id: i32, x: f32, z: f32) -> bool {
    request_at(global, unit_id, x, crate::units::ground_height(x, z), z)
}

fn request_at(global: &Global, unit_id: i32, x: f32, y: f32, z: f32) -> bool {
    crate::jump::start(global, unit_id, x, y, z) == crate::jump::Outcome::Started
}
