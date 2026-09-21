use alloc::vec::Vec;

use crate::units;

pub struct Impulses {
    pending: Vec<Pending>,
}

struct Pending {
    unit_id: i32,
    x: f32,
    y: f32,
    z: f32,
}

impl Impulses {
    pub fn new() -> Self {
        Self {
            pending: Vec::new(),
        }
    }

    pub fn damaged(
        &mut self,
        unit_id: i32,
        unit_def_id: i32,
        weapon_def_id: i32,
        attacker_id: i32,
    ) {
        let magnitude = weapon_impulse(weapon_def_id);
        let Some(magnitude) = magnitude else {
            return;
        };
        if !units::valid(attacker_id) {
            return;
        }
        let (Some(target), Some(attacker)) = (
            units::mid_position(unit_id),
            units::mid_position(attacker_id),
        ) else {
            return;
        };
        let x = target.x - attacker.x;
        let mut y = target.y - attacker.y;
        let z = target.z - attacker.z;
        let mut magnitude = magnitude;
        if y > 5.0 && magnitude > 0.0 {
            y = 0.0;
            magnitude = 0.0;
        }
        self.add(unit_id, unit_def_id, x, y, z, magnitude);
    }

    pub fn apply(&mut self) {
        for pending in core::mem::take(&mut self.pending) {
            let _ = spring::add_unit_impulse(
                pending.unit_id,
                spring::types::Float3 {
                    x: UNSTICK_CONSTANT,
                    y: 0.0,
                    z: 0.0,
                },
                0.0,
            );
            let _ = spring::add_unit_impulse(
                pending.unit_id,
                spring::types::Float3 {
                    x: pending.x,
                    y: pending.y,
                    z: pending.z,
                },
                0.0,
            );
            let _ = spring::add_unit_impulse(
                pending.unit_id,
                spring::types::Float3 {
                    x: -UNSTICK_CONSTANT,
                    y: 0.0,
                    z: 0.0,
                },
                0.0,
            );
        }
    }
}

const GRAVITY_BASELINE: f32 = 120.0;
const UNSTICK_CONSTANT: f32 = 1.0;
const IMPULSE_SCALE: f32 = 0.0032;
const MINIMUM_PULL_DISTANCE: f32 = 75.0;
const LIFT_DIVISOR: f32 = 20.0;

impl Impulses {
    fn add(&mut self, unit_id: i32, unit_def_id: i32, x: f32, y: f32, z: f32, magnitude: f32) {
        if in_build(unit_id) {
            return;
        }
        let distance = (x * x + y * y + z * z).sqrt();
        if distance <= 0.0 {
            return;
        }
        let mass = unit_mass(unit_def_id);
        if mass <= 0.0 {
            return;
        }
        let mut scale = magnitude * GRAVITY_BASELINE / distance * IMPULSE_SCALE / mass;
        let radius = spring::units_info::get_unit_radius(unit_id).unwrap_or(0.0);
        if distance < MINIMUM_PULL_DISTANCE.max(radius) && scale < 0.0 {
            scale = 0.0;
        }
        let lift = magnitude.abs() / (LIFT_DIVISOR * mass);
        let (x, y, z) = (x * scale, y * scale + lift, z * scale);
        if let Some(existing) = self
            .pending
            .iter_mut()
            .find(|pending| pending.unit_id == unit_id)
        {
            existing.x += x;
            existing.y += y;
            existing.z += z;
            return;
        }
        self.pending.push(Pending { unit_id, x, y, z });
    }
}

fn weapon_impulse(weapon_def_id: i32) -> Option<f32> {
    if let Some(value) = spring::weapon_defs::get_weapon_def_custom_param(weapon_def_id, "impulse")
        .ok()
        .and_then(|value| value.parse().ok())
    {
        return Some(value);
    }
    crate::weapons::IMPULSE_WEAPONS
        .iter()
        .find(|weapon| spring::weapon_defs::get_weapon_def_id(weapon.name) == Ok(weapon_def_id))
        .map(|weapon| weapon.impulse)
}

fn unit_mass(unit_def_id: i32) -> f32 {
    spring::get_unit_def_param_float(unit_def_id, "mass").unwrap_or(0.0)
}

fn in_build(unit_id: i32) -> bool {
    spring::units_info::get_unit_is_being_built(unit_id).unwrap_or(false)
}
