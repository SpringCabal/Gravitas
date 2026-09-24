use spring::types::Float3;

use crate::state::Global;
use crate::tests::harness::{self, Case, Step, fail};
use crate::tests::record;

pub struct PlateAndGate {
    plate: i32,
    gate: i32,
    toggler: i32,
    park: Float3,
    plate_piece: Option<Float3>,
    gate_piece: Option<Float3>,
}

impl PlateAndGate {
    pub fn new() -> Self {
        Self {
            plate: -1,
            gate: -1,
            toggler: -1,
            park: Float3::default(),
            plate_piece: None,
            gate_piece: None,
        }
    }
}

impl Case for PlateAndGate {
    fn name(&self) -> &'static str {
        "plate"
    }

    fn step(&mut self, global: &Global, frame: i32) -> Step {
        match frame {
            0 => self.press(global),
            PRESS_VERDICT_FRAME => self.check_pressed(global),
            RELEASE_VERDICT_FRAME => self.check_released(global),
            _ => Step::Wait,
        }
    }
}

const PRESS_VERDICT_FRAME: i32 = 45;
const RELEASE_VERDICT_FRAME: i32 = 110;
const PLATE_PIECE: &str = "button";
const GATE_PIECE: &str = "Gate";
const PIECE_DISPLACEMENT: f32 = 1.0;

impl PlateAndGate {
    fn press(&mut self, global: &Global) -> Step {
        let Some((plate, gate, toggler)) =
            harness::arena(global, |arena| (arena.plate, arena.gate, arena.toggler))
        else {
            return fail("no arena");
        };
        let (Some(plate_position), Some(park)) =
            (harness::position(plate), harness::position(toggler))
        else {
            return fail("no position for plate or crate");
        };
        self.plate = plate;
        self.gate = gate;
        self.toggler = toggler;
        self.park = park;
        self.plate_piece = harness::piece_position(plate, PLATE_PIECE);
        self.gate_piece = harness::piece_position(gate, GATE_PIECE);
        if self.plate_piece.is_none() || self.gate_piece.is_none() {
            return fail("plate or gate has no moving piece");
        }
        record::clear_toggles(global);
        harness::place(toggler, plate_position);
        Step::Wait
    }

    fn check_pressed(&mut self, global: &Global) -> Step {
        if !record::toggled_to(global, self.plate, true) {
            return fail("crate on the plate did not press it");
        }
        if !record::toggled_to(global, self.gate, true) {
            return fail("pressed plate did not open its linked gate");
        }
        if !harness::moved(
            self.plate_piece,
            harness::piece_position(self.plate, PLATE_PIECE),
            PIECE_DISPLACEMENT,
        ) {
            return fail("pressed plate did not animate");
        }
        if !harness::moved(
            self.gate_piece,
            harness::piece_position(self.gate, GATE_PIECE),
            PIECE_DISPLACEMENT,
        ) {
            return fail("opened gate did not animate");
        }
        record::clear_toggles(global);
        harness::place(self.toggler, self.park);
        Step::Wait
    }

    fn check_released(&mut self, global: &Global) -> Step {
        if !record::toggled_to(global, self.plate, false) {
            return fail("plate stayed pressed after the crate left");
        }
        if !record::toggled_to(global, self.gate, false) {
            return fail("gate stayed open after the plate released");
        }
        Step::Pass
    }
}
