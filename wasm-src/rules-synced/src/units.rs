use alloc::string::String;
use alloc::vec::Vec;

use spring::commands::{self, Pending};
use spring::generated::owned::unit_control;
use spring::types::Float3;

use crate::level::Spawn;

pub const RULES_PARAM_PRIVATE: i32 = 1;

pub const WEAPON_PUSH: i32 = 1;
pub const WEAPON_PULL: i32 = 2;

const DEFAULT_FACING: i32 = 0;
const DEGREES_TO_RADIANS: f32 = core::f32::consts::PI / 180.0;
const ANY_ALLEGIANCE: i32 = -1;

pub fn ground_height(x: f32, z: f32) -> f32 {
    spring::get_ground_height(x, z).unwrap_or(0.0)
}

/// Create a scenario unit, once this callin returns. Its rules params are
/// handed to the unit setup gadget through `pending_rules`, so they are set
/// before any other gadget's UnitCreated sees the unit.
pub fn spawn(spawn: &'static Spawn) -> Pending<Option<i32>> {
    let def_id = crate::defs::get().id(spawn.def_name);
    commands::queue(move || {
        let def_id = def_id?;
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
        crate::with_global(|global| global.pending_rules.set(spawn.rules));
        let created =
            unit_control::create_unit(&unit_def, position, DEFAULT_FACING, spawn.team, options);
        // Taken by UnitCreated; cleared here too in case there was none.
        crate::with_global(|global| global.pending_rules.set(&[]));
        let unit_id = created.ok()?;
        if unit_id < 0 {
            return None;
        }
        face(unit_id, spawn.angle);
        // Queued like the orders UnitCreated gave, and after them.
        order(unit_id, spring::cmd::FIRE_STATE, &[spawn.fire_state as f32]);
        order(unit_id, spring::cmd::MOVE_STATE, &[spawn.move_state as f32]);
        Some(unit_id)
    })
}

/// Rules params by name, as the scenario sets them.
pub type RulesParams = &'static [(&'static str, f32)];

/// Set a new unit's scenario rules params.
pub fn set_rules_params(unit_id: i32, rules: RulesParams) {
    for (name, value) in rules {
        crate::diagnostics::check(
            spring::set_unit_rules_param_f32(unit_id, name, *value, RULES_PARAM_PRIVATE),
            || alloc::format!("rules param {name} for unit {unit_id}"),
        );
    }
}

/// Create a unit once this callin returns; `.then` gets its id.
pub fn create(def_id: i32, position: Float3, team: i32) -> Pending<Option<i32>> {
    let unit_def = unit_control::DefRef {
        name: crate::defs::get().unit(def_id).name.clone(),
        id: def_id,
    };
    let options = unit_control::CreateUnitOptions {
        build: false,
        flatten_ground: false,
        unit_id: -1,
        builder_id: -1,
    };
    commands::queue(move || {
        let unit_id =
            unit_control::create_unit(&unit_def, position, DEFAULT_FACING, team, options).ok()?;
        (unit_id >= 0).then_some(unit_id)
    })
}

/// Give `unit_id` an order once this callin returns. The engine refusing one
/// is logged.
pub fn order(unit_id: i32, command: i32, params: &[f32]) {
    let _ = order_with(unit_id, command, params, 0);
}

/// [`order`] with option bits (`spring::cmd::options`).
pub fn order_with(unit_id: i32, command: i32, params: &[f32], options: u32) -> Pending<()> {
    let params = params.to_vec();
    commands::queue(move || order_now(unit_id, command, &params, options))
}

/// [`order_with`] right away, for code that already runs as a command. A unit
/// that died since the order was decided gets none.
pub fn order_now(unit_id: i32, command: i32, params: &[f32], options: u32) {
    if !valid(unit_id) {
        return;
    }
    crate::diagnostics::check(
        spring::give_order_to_unit(unit_id, command, params, options, 0),
        || alloc::format!("order {command} {params:?} for unit {unit_id}"),
    );
}

/// Switch the engine's movement of `unit_id` off (`enable`) or back on.
pub fn move_ctrl(unit_id: i32, enable: bool) {
    crate::diagnostics::check(spring::move_ctrl::move_ctrl(unit_id, enable), || {
        alloc::format!("move_ctrl({enable}) for unit {unit_id}")
    });
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
    spring::get_units_in_cylinder(x, z, radius, ANY_ALLEGIANCE).unwrap_or_default()
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

pub fn valid(unit_id: i32) -> bool {
    spring::valid_unit_id(unit_id).unwrap_or(false)
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

fn face(unit_id: i32, angle: f32) {
    face_radians(unit_id, angle * DEGREES_TO_RADIANS);
}
