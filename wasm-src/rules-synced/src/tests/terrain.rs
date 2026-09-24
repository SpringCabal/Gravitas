use crate::state::Global;
use crate::tests::harness::{self, Case, Step, fail};
use crate::units;

pub struct Terrain {
    gate: i32,
    base: f32,
    raised: f32,
}

impl Terrain {
    pub fn new() -> Self {
        Self {
            gate: -1,
            base: 0.0,
            raised: 0.0,
        }
    }
}

impl Case for Terrain {
    fn name(&self) -> &'static str {
        "terrain"
    }

    fn step(&mut self, global: &Global, frame: i32) -> Step {
        match frame {
            CLOSED_FRAME => self.check_closed(global),
            OPEN_FRAME => self.check_open(),
            SHUT_AGAIN_FRAME => self.check_shut_again(),
            _ => Step::Wait,
        }
    }
}

const CLOSED_FRAME: i32 = 1;
const OPEN_FRAME: i32 = 40;
const SHUT_AGAIN_FRAME: i32 = 100;
const LEVEL_MARGIN: f32 = 5.0;

impl Terrain {
    fn check_closed(&mut self, global: &Global) -> Step {
        let Some(gate) = harness::arena(global, |arena| arena.gate) else {
            return fail("no arena");
        };
        let Some(position) = harness::position(gate) else {
            return fail("no gate position");
        };
        self.gate = gate;
        self.base = position.y;
        self.raised = units::ground_height(position.x, position.z);
        if self.raised <= self.base + LEVEL_MARGIN {
            return Step::Fail(alloc::format!(
                "a closed gate did not raise the ground: base {} ground {}",
                self.base,
                self.raised
            ));
        }
        Step::Wait
    }

    fn check_open(&mut self) -> Step {
        let Some(position) = harness::position(self.gate) else {
            return fail("no gate position");
        };
        let ground = units::ground_height(position.x, position.z);
        if ground >= self.raised - LEVEL_MARGIN {
            return Step::Fail(alloc::format!(
                "an open gate did not lower the ground: still {ground} of {}",
                self.raised
            ));
        }
        Step::Wait
    }

    fn check_shut_again(&mut self) -> Step {
        let Some(position) = harness::position(self.gate) else {
            return fail("no gate position");
        };
        let ground = units::ground_height(position.x, position.z);
        if ground <= self.base + LEVEL_MARGIN {
            return Step::Fail(alloc::format!(
                "a gate that shut again did not restore the ground: {ground} of {}",
                self.raised
            ));
        }
        Step::Pass
    }
}
