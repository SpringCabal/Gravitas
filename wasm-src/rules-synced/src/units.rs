use core::cell::RefCell;

use alloc::string::String;
use alloc::vec::Vec;

use spring::generated::owned::unit_control;
use spring::types::Float3;

use crate::level::Spawn;

pub const GRAVIT: &str = "gravit";
pub const PLATE: &str = "plate";

pub const RULES_PARAM_PRIVATE: i32 = 1;

pub const WEAPON_PUSH: i32 = 1;
pub const WEAPON_PULL: i32 = 2;

const DEFAULT_FACING: i32 = 0;
const DEGREES_TO_RADIANS: f32 = core::f32::consts::PI / 180.0;
const ANY_ALLEGIANCE: i32 = -1;

thread_local! {
    static CUSTOM_PARAM_KEYS: RefCell<Vec<(i32, Vec<String>)>> = const { RefCell::new(Vec::new()) };
}

pub fn def_id(name: &str) -> Option<i32> {
    spring::get_unit_def_id_by_name(name).ok()
}

pub fn ground_height(x: f32, z: f32) -> f32 {
    spring::get_ground_height(x, z).unwrap_or(0.0)
}

pub fn spawn(spawn: &Spawn) -> Option<i32> {
    let def_id = def_id(spawn.def_name)?;
    let unit_def = unit_control::DefRef {
        name: String::from(spawn.def_name),
        id: def_id,
    };
    let options = unit_control::CreateUnitOptions {
        build: false,
        flatten_ground: true,
        unit_id: -1,
        builder_id: -1,
    };
    let position = Float3 {
        x: spawn.x,
        y: spawn.y,
        z: spawn.z,
    };
    let unit_id =
        unit_control::create_unit(&unit_def, position, DEFAULT_FACING, spawn.team, options).ok()?;
    if unit_id < 0 {
        return None;
    }
    face(unit_id, spawn.angle);
    for (name, value) in spawn.rules {
        let _ = spring::set_unit_rules_param_f32(unit_id, name, *value, RULES_PARAM_PRIVATE);
    }
    let _ = spring::give_order_to_unit(
        unit_id,
        spring::cmd::FIRE_STATE,
        &[spawn.fire_state as f32],
        0,
        0,
    );
    let _ = spring::give_order_to_unit(
        unit_id,
        spring::cmd::MOVE_STATE,
        &[spawn.move_state as f32],
        0,
        0,
    );
    Some(unit_id)
}

pub fn create(name: &str, position: Float3, team: i32) -> Option<i32> {
    let def_id = def_id(name)?;
    let unit_def = unit_control::DefRef {
        name: String::from(name),
        id: def_id,
    };
    let options = unit_control::CreateUnitOptions {
        build: false,
        flatten_ground: false,
        unit_id: -1,
        builder_id: -1,
    };
    let unit_id =
        unit_control::create_unit(&unit_def, position, DEFAULT_FACING, team, options).ok()?;
    (unit_id >= 0).then_some(unit_id)
}

pub fn position(unit_id: i32) -> Option<Float3> {
    spring::units_info::get_unit_position(
        unit_id,
        spring::units_info::GetUnitPositionOptions {
            mid_pos: false,
            aim_pos: false,
        },
    )
    .ok()
}

pub fn mid_position(unit_id: i32) -> Option<Float3> {
    spring::units_info::get_unit_position(
        unit_id,
        spring::units_info::GetUnitPositionOptions {
            mid_pos: true,
            aim_pos: false,
        },
    )
    .ok()
}

pub fn units_in_cylinder(x: f32, z: f32, radius: f32) -> Vec<i32> {
    let mut units = units_in_rectangle(x - radius, z - radius, x + radius, z + radius);
    units.retain(|unit_id| within_radius(*unit_id, x, z, radius));
    units
}

pub fn units_in_rectangle(x1: f32, z1: f32, x2: f32, z2: f32) -> Vec<i32> {
    spring::get_units_in_rectangle(x1, z1, x2, z2, ANY_ALLEGIANCE).unwrap_or_default()
}

pub fn units_in_sphere(centre: Float3, radius: f32) -> Vec<i32> {
    spring::get_units_in_sphere(centre, radius, ANY_ALLEGIANCE).unwrap_or_default()
}

pub fn units_in_box(low: Float3, high: Float3) -> Vec<i32> {
    spring::get_units_in_box(low, high, ANY_ALLEGIANCE).unwrap_or_default()
}

pub fn def_of(unit_id: i32) -> Option<i32> {
    spring::units_info::get_unit_def_id(unit_id).ok()
}

pub fn custom_param(def_id: i32, key: &str) -> Option<String> {
    spring::unit_defs::get_unit_def_custom_param(def_id, key).ok()
}

pub fn has_custom_param(def_id: i32, key: &str) -> bool {
    CUSTOM_PARAM_KEYS.with(|cache| {
        let mut cache = cache.borrow_mut();
        if let Some((_, keys)) = cache.iter().find(|(id, _)| *id == def_id) {
            return keys.iter().any(|candidate| candidate == key);
        }
        let keys = spring::unit_defs::get_unit_def_custom_param_keys(def_id).unwrap_or_default();
        let present = keys.iter().any(|candidate| candidate == key);
        cache.push((def_id, keys));
        present
    })
}

pub fn custom_param_is_one(def_id: i32, key: &str) -> bool {
    custom_param(def_id, key).is_some_and(|value| value.parse::<f32>() == Ok(1.0))
}

pub fn game_frame() -> i32 {
    spring::game::frame().map_or(0, |frame| frame as i32)
}

pub fn valid(unit_id: i32) -> bool {
    spring::valid_unit_id(unit_id).unwrap_or(false)
}

pub fn def_name(unit_def_id: i32) -> Option<String> {
    spring::unit_defs::get_unit_def_name_bytes(unit_def_id)
        .ok()
        .and_then(|bytes| String::from_utf8(bytes).ok())
}

pub fn face_radians(unit_id: i32, radians: f32) {
    let front = Float3 {
        x: radians.sin(),
        y: 0.0,
        z: radians.cos(),
    };
    let right = Float3 {
        x: -front.z,
        y: 0.0,
        z: front.x,
    };
    let _ = spring::set_unit_direction(unit_id, front, right);
}

fn within_radius(unit_id: i32, x: f32, z: f32, radius: f32) -> bool {
    let Some(position) = position(unit_id) else {
        return false;
    };
    let dx = position.x - x;
    let dz = position.z - z;
    dx * dx + dz * dz <= radius * radius
}

fn face(unit_id: i32, angle: f32) {
    face_radians(unit_id, angle * DEGREES_TO_RADIANS);
}
