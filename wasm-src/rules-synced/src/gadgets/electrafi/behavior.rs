use alloc::vec::Vec;

use crate::hazard::{Exposures, Fields, Visual};
use crate::units;

pub struct Electrafi {
    fields: Fields,
    exposures: Exposures,
}

pub struct Shock {
    pub unit_id: i32,
    pub attacker_id: i32,
    pub damage: Option<f32>,
    pub paralyze: bool,
    pub intensity: f32,
}

pub const SIZE_PARAM: &str = "elecSize";

pub const DEFAULT_RADIUS: f32 = 100.0;
const FALLBACK_RADIUS: f32 = 200.0;
pub const PROXIMITY_MARGIN: f32 = 250.0;

impl Default for Electrafi {
    fn default() -> Self {
        Self {
            fields: Fields::new(SIZE_PARAM, DEFAULT_RADIUS),
            exposures: Exposures::default(),
        }
    }
}

impl Electrafi {
    pub fn created(
        &mut self,
        spring: &spring::Spring<'_, spring::RulesSynced>,
        unit_id: i32,
        unit_def_id: i32,
    ) {
        if crate::defs::get(spring).electrafi == Some(unit_def_id) {
            self.fields.add(spring, unit_id);
        }
    }

    pub fn destroyed(&mut self, unit_id: i32) {
        self.fields.remove(unit_id);
        self.exposures.remove(unit_id);
    }

    pub fn update(
        &mut self,
        spring: &spring::Spring<'_, spring::RulesSynced>,
        frame: i32,
    ) -> Vec<Shock> {
        self.gather(spring, frame);
        self.exposures
            .resolve(frame)
            .into_iter()
            .map(|contact| Shock {
                unit_id: contact.unit_id,
                attacker_id: contact.attacker_id,
                damage: (contact.inside || !contact.robot).then_some(contact.damage),
                paralyze: contact.inside && contact.robot,
                intensity: contact.strength,
            })
            .collect()
    }

    pub fn arcs(
        &mut self,
        spring: &spring::Spring<'_, spring::RulesSynced>,
        frame: i32,
    ) -> Vec<Visual> {
        self.fields.refresh(spring, frame, FALLBACK_RADIUS)
    }

    fn gather(&mut self, spring: &spring::Spring<'_, spring::RulesSynced>, frame: i32) {
        for field in self.fields.iter() {
            let Some(centre) = units::position(spring, field.unit_id) else {
                continue;
            };
            for unit_id in units::units_in_sphere(spring, centre, field.radius + PROXIMITY_MARGIN) {
                let Some(def_id) = units::def_of(spring, unit_id) else {
                    continue;
                };
                let def = crate::defs::get(spring).unit(def_id);
                if !def.player && !def.robot {
                    continue;
                }
                let proximity = proximity_inside(spring, unit_id, centre, field.radius);
                self.exposures
                    .record(unit_id, def.robot, proximity, field.unit_id, frame);
            }
        }
    }
}

fn proximity_inside(
    spring: &spring::Spring<'_, spring::RulesSynced>,
    unit_id: i32,
    centre: spring::Float3,
    radius: f32,
) -> f32 {
    let Some(position) = units::position(spring, unit_id) else {
        return 0.0;
    };
    let dx = position.x - centre.x;
    let dy = position.y - centre.y;
    let dz = position.z - centre.z;
    let squared = dx * dx + dy * dy + dz * dz;
    if squared <= radius * radius {
        return radius - squared.sqrt();
    }
    0.0
}
