use alloc::vec::Vec;

use spring::types::Float3;

use crate::state::Global;
use crate::tests::harness::{self, Case, Rest, STILL, Step, fail};
use crate::units;

/// A closed gate raises the ground under it, opening lowers it and shutting
/// raises it again. The case opens its own gate with its own plate.
pub struct Terrain {
    plate: i32,
    gate: i32,
    toggler: i32,
    plate_at: Float3,
    gate_at: Float3,
    park: Float3,
    base: f32,
    raised: f32,
    stage: Stage,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Stage {
    Closed,
    Opening,
    Shutting,
}

impl Terrain {
    pub fn new() -> Self {
        Self {
            plate: -1,
            gate: -1,
            toggler: -1,
            plate_at: Float3::default(),
            gate_at: Float3::default(),
            park: Float3::default(),
            base: 0.0,
            raised: 0.0,
            stage: Stage::Closed,
        }
    }
}

impl Case for Terrain {
    fn name(&self) -> &'static str {
        "terrain"
    }

    fn step(&mut self, global: &Global, frame: i32) -> Step {
        match self.stage {
            Stage::Closed if frame >= CLOSED_FRAME => self.check_closed(global),
            Stage::Opening => self.wait_open(frame),
            Stage::Shutting => self.wait_shut(frame),
            _ => Step::Wait,
        }
    }

    fn rest(&self) -> Vec<Rest> {
        alloc::vec![
            Rest::at(self.plate, self.plate_at, STILL, "the terrain case's plate"),
            Rest::at(self.gate, self.gate_at, STILL, "the terrain case's gate"),
            Rest::at(self.toggler, self.park, STILL, "the terrain case's crate"),
        ]
    }
}

const CLOSED_FRAME: i32 = 1;
/// A pressed plate opens its gate, and a released one shuts it, well within
/// this many frames.
const TOGGLE_FRAMES: i32 = 60;
const LEVEL_MARGIN: f32 = 5.0;

impl Terrain {
    fn check_closed(&mut self, global: &Global) -> Step {
        let Some((plate, gate, toggler)) = harness::arena(global, |arena| {
            (
                arena.terrain_plate,
                arena.terrain_gate,
                arena.terrain_toggler,
            )
        }) else {
            return fail("no arena");
        };
        let (Some(plate_at), Some(gate_at), Some(park)) = (
            harness::position(plate),
            harness::position(gate),
            harness::position(toggler),
        ) else {
            return fail("no position for plate, gate or crate");
        };
        self.plate = plate;
        self.gate = gate;
        self.toggler = toggler;
        self.plate_at = plate_at;
        self.gate_at = gate_at;
        self.park = park;
        self.base = gate_at.y;
        self.raised = units::ground_height(gate_at.x, gate_at.z);
        if self.raised <= self.base + LEVEL_MARGIN {
            return Step::Fail(alloc::format!(
                "a closed gate did not raise the ground: base {} ground {}",
                self.base,
                self.raised
            ));
        }
        harness::place(toggler, plate_at);
        self.stage = Stage::Opening;
        Step::Wait
    }

    fn wait_open(&mut self, frame: i32) -> Step {
        let ground = self.ground();
        if ground < self.raised - LEVEL_MARGIN {
            harness::place(self.toggler, self.park);
            self.stage = Stage::Shutting;
            return Step::Wait;
        }
        if frame > CLOSED_FRAME + TOGGLE_FRAMES {
            return Step::Fail(alloc::format!(
                "an open gate did not lower the ground: still {ground} of {}",
                self.raised
            ));
        }
        Step::Wait
    }

    fn wait_shut(&mut self, frame: i32) -> Step {
        let ground = self.ground();
        if ground > self.base + LEVEL_MARGIN {
            return Step::Pass;
        }
        if frame > CLOSED_FRAME + 2 * TOGGLE_FRAMES {
            return Step::Fail(alloc::format!(
                "a gate that shut again did not restore the ground: {ground} of {}",
                self.raised
            ));
        }
        Step::Wait
    }

    fn ground(&self) -> f32 {
        units::ground_height(self.gate_at.x, self.gate_at.z)
    }
}
