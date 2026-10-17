use alloc::string::String;
use alloc::vec::Vec;

use spring::Float3;
use spring::commands::Pending;
use spring::unit_control;

use crate::gadgets::level::data::Spawn;

pub const RULES_PARAM_PRIVATE: i32 = 1;

pub const WEAPON_PUSH: i32 = 1;
pub const WEAPON_PULL: i32 = 2;

const DEFAULT_FACING: i32 = 0;
const DEGREES_TO_RADIANS: f32 = core::f32::consts::PI / 180.0;
const ANY_ALLEGIANCE: i32 = -1;

pub fn ground_height(spring: &spring::Spring<'_, spring::RulesSynced>, x: f32, z: f32) -> f32 {
    spring.terrain().get_ground_height(x, z).unwrap_or(0.0)
}

pub fn spawn(
    spring: &spring::Spring<'_, spring::RulesSynced>,
    spawn: &'static Spawn,
) -> Pending<spring::RulesSynced, Option<i32>> {
    let def_id = crate::defs::get(spring).id(spawn.def_name);
    crate::with_addon_context(|ctx| {
        ctx.commands().queue(move |spring| {
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
            let created = spring.unit_control().create_unit(
                &unit_def,
                position,
                DEFAULT_FACING,
                spawn.team,
                options,
            );
            crate::with_global(|global| global.pending_rules.set(&[]));
            let unit_id = created.ok()?;
            if unit_id < 0 {
                return None;
            }
            face(spring, unit_id, spawn.angle);
            order(
                spring,
                unit_id,
                spring::cmd::FIRE_STATE,
                &[spawn.fire_state as f32],
            );
            order(
                spring,
                unit_id,
                spring::cmd::MOVE_STATE,
                &[spawn.move_state as f32],
            );
            Some(unit_id)
        })
    })
}

pub type RulesParams = &'static [(&'static str, f32)];

pub fn set_rules_params(
    spring: &spring::Spring<'_, spring::RulesSynced>,
    unit_id: i32,
    rules: RulesParams,
) {
    for (name, value) in rules {
        crate::diagnostics::check(
            spring,
            spring.rules_params().set_unit_rules_param_f32(
                unit_id,
                name,
                *value,
                RULES_PARAM_PRIVATE,
            ),
            || alloc::format!("rules param {name} for unit {unit_id}"),
        );
    }
}

pub fn create(
    spring: &spring::Spring<'_, spring::RulesSynced>,
    def_id: i32,
    position: Float3,
    team: i32,
) -> Pending<spring::RulesSynced, Option<i32>> {
    let unit_def = unit_control::DefRef {
        name: crate::defs::get(spring).unit(def_id).name.clone(),
        id: def_id,
    };
    let options = unit_control::CreateUnitOptions {
        build: false,
        flatten_ground: false,
        unit_id: -1,
        builder_id: -1,
    };
    crate::with_addon_context(|ctx| {
        ctx.commands().queue(move |spring| {
            let unit_id = spring
                .unit_control()
                .create_unit(&unit_def, position, DEFAULT_FACING, team, options)
                .ok()?;
            (unit_id >= 0).then_some(unit_id)
        })
    })
}

pub fn order(
    spring: &spring::Spring<'_, spring::RulesSynced>,
    unit_id: i32,
    command: i32,
    params: &[f32],
) {
    let _ = order_with(spring, unit_id, command, params, 0);
}

pub fn order_with(
    _spring: &spring::Spring<'_, spring::RulesSynced>,
    unit_id: i32,
    command: i32,
    params: &[f32],
    options: u32,
) -> Pending<spring::RulesSynced, ()> {
    let params = params.to_vec();
    crate::with_addon_context(|ctx| {
        ctx.commands()
            .queue(move |spring| order_now(spring, unit_id, command, &params, options))
    })
}

pub fn order_now(
    spring: &spring::Spring<'_, spring::RulesSynced>,
    unit_id: i32,
    command: i32,
    params: &[f32],
    options: u32,
) {
    if !valid(spring, unit_id) {
        return;
    }
    crate::diagnostics::check(
        spring,
        spring
            .unit_control()
            .give_order_to_unit(unit_id, command, params, options, 0),
        || alloc::format!("order {command} {params:?} for unit {unit_id}"),
    );
}

pub fn move_ctrl(spring: &spring::Spring<'_, spring::RulesSynced>, unit_id: i32, enable: bool) {
    crate::diagnostics::check(
        spring,
        spring.move_ctrl().move_ctrl(unit_id, enable),
        || alloc::format!("move_ctrl({enable}) for unit {unit_id}"),
    );
}

pub fn position(spring: &spring::Spring<'_, spring::RulesSynced>, unit_id: i32) -> Option<Float3> {
    spring
        .units_info()
        .get_unit_position(
            unit_id,
            spring::units_info::GetUnitPositionOptions {
                mid_pos: false,
                aim_pos: false,
            },
        )
        .ok()
}

pub fn mid_position(
    spring: &spring::Spring<'_, spring::RulesSynced>,
    unit_id: i32,
) -> Option<Float3> {
    spring
        .units_info()
        .get_unit_position(
            unit_id,
            spring::units_info::GetUnitPositionOptions {
                mid_pos: true,
                aim_pos: false,
            },
        )
        .ok()
}

pub fn units_in_cylinder(
    spring: &spring::Spring<'_, spring::RulesSynced>,
    x: f32,
    z: f32,
    radius: f32,
) -> Vec<i32> {
    spring
        .units_query()
        .get_units_in_cylinder(x, z, radius, ANY_ALLEGIANCE)
        .unwrap_or_default()
}

pub fn units_in_rectangle(
    spring: &spring::Spring<'_, spring::RulesSynced>,
    x1: f32,
    z1: f32,
    x2: f32,
    z2: f32,
) -> Vec<i32> {
    spring
        .units_query()
        .get_units_in_rectangle(x1, z1, x2, z2, ANY_ALLEGIANCE)
        .unwrap_or_default()
}

pub fn units_in_sphere(
    spring: &spring::Spring<'_, spring::RulesSynced>,
    centre: Float3,
    radius: f32,
) -> Vec<i32> {
    spring
        .units_query()
        .get_units_in_sphere(centre.x, centre.y, centre.z, radius, ANY_ALLEGIANCE)
        .unwrap_or_default()
}

pub fn units_in_box(
    spring: &spring::Spring<'_, spring::RulesSynced>,
    low: Float3,
    high: Float3,
) -> Vec<i32> {
    spring
        .units_query()
        .get_units_in_box(low.x, low.y, low.z, high.x, high.y, high.z, ANY_ALLEGIANCE)
        .unwrap_or_default()
}

pub fn def_of(spring: &spring::Spring<'_, spring::RulesSynced>, unit_id: i32) -> Option<i32> {
    spring.units_info().get_unit_def_id(unit_id).ok()
}

pub fn valid(spring: &spring::Spring<'_, spring::RulesSynced>, unit_id: i32) -> bool {
    spring.units_query().valid_unit_id(unit_id).unwrap_or(false)
}

pub fn face_radians(spring: &spring::Spring<'_, spring::RulesSynced>, unit_id: i32, radians: f32) {
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
    let _ = spring
        .unit_control()
        .set_unit_direction(unit_id, front, right);
}

fn face(spring: &spring::Spring<'_, spring::RulesSynced>, unit_id: i32, angle: f32) {
    face_radians(spring, unit_id, angle * DEGREES_TO_RADIANS);
}
