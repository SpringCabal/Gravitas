use alloc::vec::Vec;

use crate::units;

pub struct Fires {
    fields: Vec<Field>,
    burning: Vec<Burning>,
    extinguished: Vec<i32>,
}

struct Field {
    unit_id: i32,
    radius: f32,
}

struct Burning {
    unit_id: i32,
    proximity: f32,
    attacker_id: i32,
    caught_frame: i32,
}

impl Fires {
    pub fn new() -> Self {
        Self {
            fields: Vec::new(),
            burning: Vec::new(),
            extinguished: Vec::new(),
        }
    }

    pub fn created(&mut self, unit_id: i32, unit_def_id: i32) {
        if units::def_id(FIRE) != Some(unit_def_id) {
            return;
        }
        let radius = match spring::get_unit_rules_param_f32(unit_id, "fireSize") {
            Ok(radius) => radius,
            Err(_) => {
                let _ = spring::set_unit_rules_param_f32(
                    unit_id,
                    "fireSize",
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
        self.burning.retain(|burning| burning.unit_id != unit_id);
    }

    pub fn update(&mut self, frame: i32) -> Vec<Burn> {
        self.gather(frame);
        self.resolve(frame)
    }

    pub fn flames(&mut self, frame: i32) -> Vec<Flame> {
        if frame % DRAW_INTERVAL != 0 {
            return Vec::new();
        }
        let mut flames = Vec::new();
        for field in &mut self.fields {
            let Some(position) = units::position(field.unit_id) else {
                continue;
            };
            field.radius = spring::get_unit_rules_param_f32(field.unit_id, "fireSize")
                .unwrap_or(DEFAULT_RADIUS);
            flames.push(Flame {
                position,
                radius: field.radius,
            });
        }
        flames
    }

    pub fn take_extinguished(&mut self) -> Vec<i32> {
        core::mem::take(&mut self.extinguished)
    }
}

pub struct Burn {
    pub unit_id: i32,
    pub attacker_id: i32,
    pub damage: f32,
    pub size_mult: f32,
}

pub struct Flame {
    pub position: spring::types::Float3,
    pub radius: f32,
}

pub const FIRE: &str = "fire";

const DEFAULT_RADIUS: f32 = 500.0;
const FIRE_SPEED: f32 = 0.5;
const REACH_FACTOR: f32 = 1.7;
const EMIT_ROTATION_SPREAD: f32 = (8.0 / 360.0) * core::f32::consts::TAU;
const DRAW_INTERVAL: i32 = 15;
const BURN_FRAMES: i32 = 60;
const BASE_DAMAGE: f32 = 2.0;
const PROXIMITY_DAMAGE_SCALE: f32 = 0.1;

impl Fires {
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
            let reach = radius * FIRE_SPEED * REACH_FACTOR;
            for unit_id in units::units_in_cylinder(centre.x, centre.z, reach) {
                if spring::get_unit_rules_param_f32(unit_id, "invulnerable")
                    .is_ok_and(|value| value == 1.0)
                {
                    continue;
                }
                let proximity = proximity_inside(unit_id, centre);
                self.record(unit_id, proximity, field_id, frame);
            }
        }
    }

    fn record(&mut self, unit_id: i32, proximity: f32, field_id: i32, frame: i32) {
        if let Some(burning) = self
            .burning
            .iter_mut()
            .find(|burning| burning.unit_id == unit_id)
        {
            if proximity > burning.proximity && proximity > 0.0 {
                burning.attacker_id = field_id;
            }
            burning.proximity = burning.proximity.max(proximity);
            if proximity > 0.0 {
                burning.caught_frame = frame;
            }
            return;
        }
        self.burning.push(Burning {
            unit_id,
            proximity,
            attacker_id: field_id,
            caught_frame: if proximity > 0.0 { frame } else { i32::MIN },
        });
    }

    fn resolve(&mut self, frame: i32) -> Vec<Burn> {
        let mut burns = Vec::new();
        for burning in &mut self.burning {
            if burning.proximity > 0.0 {
                burns.push(Burn {
                    unit_id: burning.unit_id,
                    attacker_id: burning.attacker_id,
                    damage: BASE_DAMAGE + PROXIMITY_DAMAGE_SCALE * burning.proximity,
                    size_mult: 1.0,
                });
            } else if burning.caught_frame + BURN_FRAMES > frame {
                burns.push(Burn {
                    unit_id: burning.unit_id,
                    attacker_id: burning.attacker_id,
                    damage: BASE_DAMAGE,
                    size_mult: (burning.caught_frame + BURN_FRAMES - frame) as f32
                        / BURN_FRAMES as f32,
                });
            }
            burning.proximity = 0.0;
        }
        for burning in &self.burning {
            if burning.caught_frame + BURN_FRAMES <= frame {
                self.extinguished.push(burning.unit_id);
            }
        }
        self.burning
            .retain(|burning| burning.caught_frame + BURN_FRAMES > frame);
        burns
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
