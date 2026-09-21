use alloc::vec::Vec;

use crate::units;

pub struct FallDamage {
    immune_until: Vec<(i32, i32)>,
}

pub struct Attributes {
    pub elasticity: f32,
    pub friction: f32,
    pub out_of_map_damage_per_elmo: f32,
    pub velocity_damage_scale: f32,
}

impl FallDamage {
    pub fn new() -> Self {
        Self {
            immune_until: Vec::new(),
        }
    }

    pub fn set_immunity(&mut self, unit_id: i32, until_frame: i32) {
        if let Some(entry) = self.immune_until.iter_mut().find(|(id, _)| *id == unit_id) {
            entry.1 = entry.1.max(until_frame);
            return;
        }
        self.immune_until.push((unit_id, until_frame));
    }

    pub fn forget(&mut self, unit_id: i32) {
        self.immune_until.retain(|(id, _)| *id != unit_id);
    }

    pub fn is_immune(&self, unit_id: i32, frame: i32) -> bool {
        self.immune_until
            .iter()
            .any(|(id, until)| *id == unit_id && *until >= frame)
    }
}

pub fn ground_collision(unit_id: i32, unit_def_id: i32) -> f32 {
    let attributes = attributes(unit_def_id);
    let velocity = spring::units_info::get_unit_velocity(unit_id).unwrap_or_default();
    let Some(position) = units::position(unit_id) else {
        return 0.0;
    };
    let normal = ground_normal(position.x, position.z);
    let normal_component = spring::types::Float3 {
        x: velocity.x * normal.x,
        y: velocity.y * normal.y,
        z: velocity.z * normal.z,
    };
    let tangent = spring::types::Float3 {
        x: velocity.x - normal_component.x,
        y: velocity.y - normal_component.y,
        z: velocity.z - normal_component.z,
    };
    let bounced = spring::types::Float3 {
        x: tangent.x * attributes.friction + normal_component.x * attributes.elasticity,
        y: tangent.y * attributes.friction + normal_component.y * attributes.elasticity,
        z: tangent.z * attributes.friction + normal_component.z * attributes.elasticity,
    };
    let _ = spring::set_unit_velocity(unit_id, spring::types::Float3::default());
    let _ = spring::add_unit_impulse(unit_id, bounced, 0.0);
    let damage_x = normal_component.x + tangent.x * TANGENT_DAMAGE;
    let damage_y = normal_component.y + tangent.y * TANGENT_DAMAGE;
    let damage_z = normal_component.z + tangent.z * TANGENT_DAMAGE;
    let speed = (damage_x * damage_x + damage_y * damage_y + damage_z * damage_z).sqrt();
    speed_to_damage(&attributes, speed) + outside_map_damage(unit_id, &attributes)
}

pub const GROUND_COLLISION: i32 = -2;
pub const OBJECT_COLLISION: i32 = -3;

const TANGENT_DAMAGE: f32 = 0.5;
const VELOCITY_DAMAGE_THRESHOLD: f32 = 3.0;
const OUT_OF_MAP_DAMAGE_DIVISOR: f32 = 800.0;
const VELOCITY_DAMAGE_MASS_FACTOR: f32 = 0.6;
const IMMOBILE_MASS_MULTIPLIER: f32 = 10.0;
const DEFAULT_ELASTICITY: f32 = 0.3;
const DEFAULT_FRICTION: f32 = 0.8;

pub fn attributes(unit_def_id: i32) -> Attributes {
    let mass = def_float(unit_def_id, "mass");
    let health = spring::get_unit_def_health(unit_def_id).unwrap_or(0.0);
    let speed = spring::get_unit_def_speed(unit_def_id).unwrap_or(0.0);
    let mut velocity_damage_scale = mass * VELOCITY_DAMAGE_MASS_FACTOR;
    if speed == 0.0 {
        velocity_damage_scale *= IMMOBILE_MASS_MULTIPLIER;
    }
    Attributes {
        elasticity: custom_float(unit_def_id, "elasticity").unwrap_or(DEFAULT_ELASTICITY),
        friction: custom_float(unit_def_id, "friction").unwrap_or(DEFAULT_FRICTION),
        out_of_map_damage_per_elmo: health / OUT_OF_MAP_DAMAGE_DIVISOR,
        velocity_damage_scale,
    }
}

pub fn speed_to_damage(attributes: &Attributes, speed: f32) -> f32 {
    if speed <= VELOCITY_DAMAGE_THRESHOLD {
        return 0.0;
    }
    (speed - VELOCITY_DAMAGE_THRESHOLD) * attributes.velocity_damage_scale
}

fn outside_map_damage(unit_id: i32, attributes: &Attributes) -> f32 {
    let Some(position) = units::position(unit_id) else {
        return 0.0;
    };
    let Ok(map) = spring::game::get_game_map_info(0) else {
        return 0.0;
    };
    let width = map.map_size_x as f32;
    let height = map.map_size_z as f32;
    let outside = (-position.x)
        .max(-position.z)
        .max(position.x - width)
        .max(position.z - height);
    if outside <= 0.0 {
        return 0.0;
    }
    outside * attributes.out_of_map_damage_per_elmo
}

fn ground_normal(x: f32, z: f32) -> spring::types::Float3 {
    let normal = spring::get_ground_normal(x, z, false)
        .map(|value| value.normal)
        .unwrap_or(spring::types::Float3 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        });
    let magnitude = (normal.x * normal.x + normal.y * normal.y + normal.z * normal.z).sqrt();
    if magnitude <= 0.0 {
        return spring::types::Float3 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        };
    }
    spring::types::Float3 {
        x: normal.x / magnitude,
        y: normal.y / magnitude,
        z: normal.z / magnitude,
    }
}

fn custom_float(unit_def_id: i32, key: &str) -> Option<f32> {
    units::custom_param(unit_def_id, key).and_then(|value| value.parse().ok())
}

fn def_float(unit_def_id: i32, key: &str) -> f32 {
    spring::get_unit_def_param_float(unit_def_id, key).unwrap_or(0.0)
}
