use alloc::vec::Vec;

use crate::units;

pub struct Electrafi {
    fields: Vec<Field>,
    watched: Vec<Watched>,
}

struct Field {
    unit_id: i32,
    radius: f32,
}

struct Watched {
    unit_id: i32,
    is_robot: bool,
    proximity: f32,
    attacker_id: i32,
    last_contact_frame: i32,
}

impl Electrafi {
    pub fn new() -> Self {
        Self {
            fields: Vec::new(),
            watched: Vec::new(),
        }
    }

    pub fn created(&mut self, unit_id: i32, unit_def_id: i32) {
        if units::def_id(ELECTRAFI) != Some(unit_def_id) {
            return;
        }
        let radius = match spring::get_unit_rules_param_f32(unit_id, "elecSize") {
            Ok(radius) => radius,
            Err(_) => {
                let _ = spring::set_unit_rules_param_f32(
                    unit_id,
                    "elecSize",
                    DEFAULT_RADIUS,
                    units::RULES_PARAM_PRIVATE,
                );
                DEFAULT_RADIUS
            }
        };
        self.fields.push(Field { unit_id, radius });
    }

    pub fn destroyed(&mut self, unit_id: i32) {
        self.fields.retain(|field| field.unit_id != unit_id);
        self.watched.retain(|watched| watched.unit_id != unit_id);
    }

    pub fn all_fields(&self) -> Vec<i32> {
        self.fields.iter().map(|field| field.unit_id).collect()
    }

    pub fn units_in_proximity(&self, field_id: i32) -> Vec<i32> {
        let Some(field) = self.fields.iter().find(|field| field.unit_id == field_id) else {
            return Vec::new();
        };
        let Some(position) = units::position(field_id) else {
            return Vec::new();
        };
        units_in_sphere(position, field.radius + PROXIMITY_MARGIN)
    }

    pub fn update(&mut self, frame: i32) -> Vec<Shock> {
        self.gather(frame);
        self.resolve(frame)
    }

    pub fn arcs(&mut self, frame: i32) -> Vec<Arc> {
        if frame % DRAW_INTERVAL != 0 {
            return Vec::new();
        }
        let mut arcs = Vec::new();
        for field in &mut self.fields {
            let Some(position) = units::position(field.unit_id) else {
                continue;
            };
            field.radius = spring::get_unit_rules_param_f32(field.unit_id, "elecSize")
                .unwrap_or(FALLBACK_RADIUS);
            arcs.push(Arc {
                position,
                radius: field.radius,
            });
        }
        arcs
    }
}

pub struct Arc {
    pub position: spring::types::Float3,
    pub radius: f32,
}

pub struct Shock {
    pub unit_id: i32,
    pub attacker_id: i32,
    pub damage: f32,
    pub paralyze: bool,
    pub intensity: f32,
}

pub const ELECTRAFI: &str = "electrafi";

const DEFAULT_RADIUS: f32 = 100.0;
const PROXIMITY_MARGIN: f32 = 250.0;
const FALLBACK_RADIUS: f32 = 200.0;
const DRAW_INTERVAL: i32 = 15;
const BURN_FRAMES: i32 = 60;
const BASE_DAMAGE: f32 = 2.0;
const PROXIMITY_DAMAGE_SCALE: f32 = 0.1;

impl Electrafi {
    fn gather(&mut self, frame: i32) {
        let fields: Vec<(i32, f32)> = self
            .fields
            .iter()
            .map(|field| (field.unit_id, field.radius))
            .collect();
        for (field_id, radius) in fields {
            let Some(centre) = units::position(field_id) else {
                continue;
            };
            for unit_id in units_in_sphere(centre, radius + PROXIMITY_MARGIN) {
                let Some(def_id) = units::def_of(unit_id) else {
                    continue;
                };
                let is_player = units::custom_param(def_id, "player").is_some();
                let is_robot = units::custom_param(def_id, "robot").is_some();
                if !is_player && !is_robot {
                    continue;
                }
                let proximity = proximity_inside(unit_id, centre, radius);
                self.record(unit_id, is_robot, proximity, field_id, frame);
            }
        }
    }

    fn record(&mut self, unit_id: i32, is_robot: bool, proximity: f32, field_id: i32, frame: i32) {
        if let Some(watched) = self
            .watched
            .iter_mut()
            .find(|watched| watched.unit_id == unit_id)
        {
            if proximity > watched.proximity && proximity > 0.0 {
                watched.attacker_id = field_id;
            }
            watched.proximity = watched.proximity.max(proximity);
            if proximity > 0.0 {
                watched.last_contact_frame = frame;
            }
            return;
        }
        self.watched.push(Watched {
            unit_id,
            is_robot,
            proximity,
            attacker_id: field_id,
            last_contact_frame: if proximity > 0.0 { frame } else { i32::MIN },
        });
    }

    fn resolve(&mut self, frame: i32) -> Vec<Shock> {
        let mut shocks = Vec::new();
        for watched in &mut self.watched {
            if watched.proximity > 0.0 {
                shocks.push(Shock {
                    unit_id: watched.unit_id,
                    attacker_id: watched.attacker_id,
                    damage: BASE_DAMAGE + PROXIMITY_DAMAGE_SCALE * watched.proximity,
                    paralyze: watched.is_robot,
                    intensity: 1.0,
                });
            } else if watched.last_contact_frame + BURN_FRAMES > frame {
                let remaining = (watched.last_contact_frame + BURN_FRAMES - frame) as f32;
                let intensity = remaining / BURN_FRAMES as f32;
                if !watched.is_robot {
                    shocks.push(Shock {
                        unit_id: watched.unit_id,
                        attacker_id: watched.attacker_id,
                        damage: BASE_DAMAGE,
                        paralyze: false,
                        intensity,
                    });
                }
            }
            watched.proximity = 0.0;
        }
        self.watched
            .retain(|watched| watched.last_contact_frame + BURN_FRAMES > frame);
        shocks
    }
}

fn proximity_inside(unit_id: i32, centre: spring::types::Float3, radius: f32) -> f32 {
    let Some(position) = units::position(unit_id) else {
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

fn units_in_sphere(centre: spring::types::Float3, radius: f32) -> Vec<i32> {
    let mut output = alloc::vec![0i32; 128];
    match spring::get_units_in_sphere_into(centre, radius, ANY_ALLEGIANCE, &mut output) {
        Ok(spring::BufferFill::Complete(count)) => {
            output.truncate(count);
            output
        }
        _ => Vec::new(),
    }
}

const ANY_ALLEGIANCE: i32 = -1;
