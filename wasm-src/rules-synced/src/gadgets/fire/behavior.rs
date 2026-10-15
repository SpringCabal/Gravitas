use alloc::vec::Vec;

use crate::hazard::{Exposures, Fields, Visual};
use crate::units;

pub struct Fires {
    fields: Fields,
    exposures: Exposures,
}

pub struct Burn {
    pub unit_id: i32,
    pub attacker_id: i32,
    pub damage: f32,
    pub size_mult: f32,
}

pub const SIZE_PARAM: &str = "fireSize";

const DEFAULT_RADIUS: f32 = 500.0;
const FIRE_SPEED: f32 = 0.5;
const REACH_FACTOR: f32 = 1.7;
const EMIT_ROTATION_SPREAD: f32 = (8.0 / 360.0) * core::f32::consts::TAU;

impl Default for Fires {
    fn default() -> Self {
        Self {
            fields: Fields::new(SIZE_PARAM, DEFAULT_RADIUS),
            exposures: Exposures::default(),
        }
    }
}

impl Fires {
    pub fn created(&mut self, unit_id: i32, unit_def_id: i32) {
        if crate::defs::get().fire == Some(unit_def_id) {
            self.fields.add(unit_id);
        }
    }

    pub fn destroyed(&mut self, unit_id: i32) {
        self.fields.remove(unit_id);
        self.exposures.remove(unit_id);
    }

    pub fn update(&mut self, frame: i32) -> Vec<Burn> {
        self.gather(frame);
        self.exposures
            .resolve(frame)
            .into_iter()
            .map(|contact| Burn {
                unit_id: contact.unit_id,
                attacker_id: contact.attacker_id,
                damage: contact.damage,
                size_mult: contact.strength,
            })
            .collect()
    }

    pub fn flames(&mut self, frame: i32) -> Vec<Visual> {
        self.fields.refresh(frame, DEFAULT_RADIUS)
    }

    fn gather(&mut self, frame: i32) {
        for field in self.fields.iter() {
            let Some(centre) = units::position(field.unit_id) else {
                continue;
            };
            let reach = field.radius * FIRE_SPEED * REACH_FACTOR;
            for unit_id in units::units_in_cylinder(centre.x, centre.z, reach) {
                if crate::effects::is_invulnerable(unit_id) {
                    continue;
                }
                let proximity = proximity_inside(unit_id, centre);
                self.exposures
                    .record(unit_id, false, proximity, field.unit_id, frame);
            }
        }
    }
}

fn proximity_inside(unit_id: i32, centre: spring::types::Float3) -> f32 {
    let Some(position) = units::position(unit_id) else {
        return 0.0;
    };
    if position.y < centre.y {
        return 0.0;
    }
    let base_angle = (EMIT_ROTATION_SPREAD.atan() / FIRE_SPEED).tan();
    let dx = position.x - centre.x;
    let dz = position.z - centre.z;
    let perpendicular = (dx * dx + dz * dz).sqrt();
    let spread = centre.y * base_angle.tan();
    let inside = spread - perpendicular;
    if inside < 0.0 {
        return 0.0;
    }
    base_angle.cos() * inside
}
