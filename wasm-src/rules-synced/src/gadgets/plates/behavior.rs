use alloc::vec::Vec;

use crate::units;

#[derive(Default)]
pub struct Plates {
    plates: Vec<Plate>,
    bitmask_links: Vec<BitmaskLink>,
    gates: Vec<Gate>,
    checks_enabled: bool,
}

struct Plate {
    unit_id: i32,
    gate_id: Option<i32>,
    state: bool,
}

struct BitmaskLink {
    gate_id: i32,
    mask: Vec<(i32, bool)>,
}

struct Gate {
    unit_id: i32,
    state: bool,
}

impl Plates {
    pub fn created(&mut self, unit_id: i32, unit_def_id: i32) -> bool {
        if crate::defs::get().plate == Some(unit_def_id) {
            self.plates.push(Plate {
                unit_id,
                gate_id: None,
                state: false,
            });
            return true;
        }
        if is_gate(unit_id) {
            self.gates.push(Gate {
                unit_id,
                state: false,
            });
        }
        false
    }

    pub fn destroyed(&mut self, unit_id: i32) {
        self.plates.retain(|plate| plate.unit_id != unit_id);
        self.bitmask_links.retain(|link| link.gate_id != unit_id);
        self.gates.retain(|gate| gate.unit_id != unit_id);
    }

    pub fn simple_link(&mut self, plate_id: i32, gate_id: i32) -> bool {
        if !is_gate(gate_id) {
            return false;
        }
        let Some(plate) = self
            .plates
            .iter_mut()
            .find(|plate| plate.unit_id == plate_id)
        else {
            return false;
        };
        plate.gate_id = Some(gate_id);
        true
    }

    pub fn bitmask_link(&mut self, mask: Vec<(i32, bool)>, gate_id: i32) -> bool {
        if !is_gate(gate_id) {
            return false;
        }
        if mask
            .iter()
            .any(|(plate_id, _)| !self.plates.iter().any(|plate| plate.unit_id == *plate_id))
        {
            return false;
        }
        self.bitmask_links.push(BitmaskLink { gate_id, mask });
        true
    }

    pub fn enable_link_checks(&mut self) {
        self.checks_enabled = true;
    }

    pub fn pending(&mut self, frame: i32) -> Vec<(i32, bool)> {
        if !self.checks_enabled || frame % UPDATE_RATE != 0 {
            return Vec::new();
        }
        let mut changes = Vec::new();
        for index in 0..self.plates.len() {
            let unit_id = self.plates[index].unit_id;
            let pressed = plate_pressed(unit_id);
            self.set_plate_state(index, pressed, &mut changes);
        }
        for index in 0..self.plates.len() {
            let Some(gate_id) = self.plates[index].gate_id else {
                continue;
            };
            let state = self.plates[index].state;
            self.set_gate_state(gate_id, state, &mut changes);
        }
        for index in 0..self.bitmask_links.len() {
            let gate_id = self.bitmask_links[index].gate_id;
            let satisfied = self.bitmask_links[index]
                .mask
                .iter()
                .all(|(plate_id, wanted)| self.plate_state(*plate_id) == *wanted);
            self.set_gate_state(gate_id, satisfied, &mut changes);
        }
        changes
    }
}

pub fn set_state(unit_id: i32, active: bool) {
    units::order(unit_id, spring::cmd::ONOFF, &[f32::from(active)]);
}

const UPDATE_RATE: i32 = 5;
const PLATE_ACTIVATION_RANGE: f32 = 40.0;

impl Plates {
    fn plate_state(&self, plate_id: i32) -> bool {
        self.plates
            .iter()
            .find(|plate| plate.unit_id == plate_id)
            .is_some_and(|plate| plate.state)
    }

    fn set_plate_state(&mut self, index: usize, state: bool, changes: &mut Vec<(i32, bool)>) {
        if self.plates[index].state == state {
            return;
        }
        self.plates[index].state = state;
        changes.push((self.plates[index].unit_id, state));
    }

    fn set_gate_state(&mut self, gate_id: i32, state: bool, changes: &mut Vec<(i32, bool)>) {
        match self.gates.iter_mut().find(|gate| gate.unit_id == gate_id) {
            Some(gate) if gate.state == state => return,
            Some(gate) => gate.state = state,
            None => self.gates.push(Gate {
                unit_id: gate_id,
                state,
            }),
        }
        changes.push((gate_id, state));
    }
}

fn is_gate(unit_id: i32) -> bool {
    units::def_of(unit_id).is_some_and(|def_id| crate::defs::get().unit(def_id).gate)
}

pub fn disable_collision(unit_id: i32) {
    let _ = spring::set_unit_collision_volume_data(
        unit_id,
        spring::types::Float3::default(),
        spring::types::Float3::default(),
        0,
        0,
        0,
    );
    let _ = spring::set_unit_blocking(
        unit_id,
        spring::generated::owned::unit_control::SetUnitBlockingOptions {
            blocking: false,
            solid_objects: false,
            projectiles: false,
            quad_map_rays: false,
            crushable: false,
            block_enemy_pushing: false,
            block_height_changes: false,
        },
    );
}

fn plate_pressed(plate_id: i32) -> bool {
    let Some(position) = units::position(plate_id) else {
        return false;
    };
    units::units_in_cylinder(position.x, position.z, PLATE_ACTIVATION_RANGE)
        .into_iter()
        .any(is_toggler)
}

fn is_toggler(unit_id: i32) -> bool {
    let Some(def_id) = units::def_of(unit_id) else {
        return false;
    };
    crate::defs::get().unit(def_id).plate_toggler
}
