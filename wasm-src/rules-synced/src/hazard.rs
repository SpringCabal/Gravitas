use alloc::vec::Vec;

use spring::types::Float3;

use crate::units;

const LINGER_FRAMES: i32 = 60;
const BASE_DAMAGE: f32 = 2.0;
const PROXIMITY_DAMAGE_SCALE: f32 = 0.1;
const DRAW_INTERVAL: i32 = 15;

pub struct Fields {
    fields: Vec<Field>,
    size_param: &'static str,
    default_size: f32,
}

#[derive(Clone, Copy)]
pub struct Field {
    pub unit_id: i32,
    pub radius: f32,
}

pub struct Visual {
    pub position: Float3,
    pub radius: f32,
}

impl Fields {
    pub const fn new(size_param: &'static str, default_size: f32) -> Self {
        Self {
            fields: Vec::new(),
            size_param,
            default_size,
        }
    }

    pub fn add(&mut self, unit_id: i32) {
        let radius = match spring::get_unit_rules_param_f32(unit_id, self.size_param) {
            Ok(radius) => radius,
            Err(_) => {
                let _ = spring::set_unit_rules_param_f32(
                    unit_id,
                    self.size_param,
                    self.default_size,
                    units::RULES_PARAM_PRIVATE,
                );
                self.default_size
            }
        };
        self.fields.push(Field { unit_id, radius });
    }

    pub fn remove(&mut self, unit_id: i32) {
        self.fields.retain(|field| field.unit_id != unit_id);
    }

    pub fn iter(&self) -> impl Iterator<Item = Field> + '_ {
        self.fields.iter().copied()
    }

    pub fn refresh(&mut self, frame: i32, fallback: f32) -> Vec<Visual> {
        if frame % DRAW_INTERVAL != 0 {
            return Vec::new();
        }
        let mut visuals = Vec::new();
        for field in &mut self.fields {
            let Some(position) = units::position(field.unit_id) else {
                continue;
            };
            field.radius = spring::get_unit_rules_param_f32(field.unit_id, self.size_param)
                .unwrap_or(fallback);
            visuals.push(Visual {
                position,
                radius: field.radius,
            });
        }
        visuals
    }
}

#[derive(Default)]
pub struct Exposures {
    exposed: Vec<Exposure>,
}

struct Exposure {
    unit_id: i32,
    robot: bool,
    proximity: f32,
    attacker_id: i32,
    last_contact_frame: i32,
}

pub struct Contact {
    pub unit_id: i32,
    pub attacker_id: i32,
    pub robot: bool,
    pub damage: f32,
    pub inside: bool,
    pub strength: f32,
}

impl Exposures {
    pub fn record(&mut self, unit_id: i32, robot: bool, proximity: f32, field_id: i32, frame: i32) {
        if let Some(exposure) = self
            .exposed
            .iter_mut()
            .find(|exposure| exposure.unit_id == unit_id)
        {
            if proximity > exposure.proximity && proximity > 0.0 {
                exposure.attacker_id = field_id;
            }
            exposure.proximity = exposure.proximity.max(proximity);
            if proximity > 0.0 {
                exposure.last_contact_frame = frame;
            }
            return;
        }
        self.exposed.push(Exposure {
            unit_id,
            robot,
            proximity,
            attacker_id: field_id,
            last_contact_frame: if proximity > 0.0 { frame } else { i32::MIN },
        });
    }

    pub fn resolve(&mut self, frame: i32) -> Vec<Contact> {
        let mut contacts = Vec::new();
        for exposure in &mut self.exposed {
            if exposure.proximity > 0.0 {
                contacts.push(Contact {
                    unit_id: exposure.unit_id,
                    attacker_id: exposure.attacker_id,
                    robot: exposure.robot,
                    damage: BASE_DAMAGE + PROXIMITY_DAMAGE_SCALE * exposure.proximity,
                    inside: true,
                    strength: 1.0,
                });
            } else if exposure.last_contact_frame + LINGER_FRAMES > frame {
                let remaining = (exposure.last_contact_frame + LINGER_FRAMES - frame) as f32;
                contacts.push(Contact {
                    unit_id: exposure.unit_id,
                    attacker_id: exposure.attacker_id,
                    robot: exposure.robot,
                    damage: BASE_DAMAGE,
                    inside: false,
                    strength: remaining / LINGER_FRAMES as f32,
                });
            }
            exposure.proximity = 0.0;
        }
        self.exposed
            .retain(|exposure| exposure.last_contact_frame + LINGER_FRAMES > frame);
        contacts
    }

    pub fn remove(&mut self, unit_id: i32) {
        self.exposed.retain(|exposure| exposure.unit_id != unit_id);
    }
}
