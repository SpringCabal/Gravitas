pub fn apply(spring: &spring::Spring<'_, spring::RulesSynced>, unit_id: i32, unit_def_id: i32) {
    let def = crate::defs::get(spring).unit(unit_def_id);
    if let Some(radius) = def.radius {
        let _ = spring
            .unit_control()
            .set_unit_radius_and_height(unit_id, radius, HEIGHT);
    }
    let offset = spring::Float3 {
        x: 0.0,
        y: AIM_HEIGHT,
        z: 0.0,
    };
    let _ = spring
        .unit_control()
        .set_unit_mid_and_aim_pos(unit_id, offset, offset, true);
}

pub fn configure_effect(
    spring: &spring::Spring<'_, spring::RulesSynced>,
    unit_id: i32,
    unit_def_id: i32,
) {
    if !crate::defs::get(spring).unit(unit_def_id).effect {
        return;
    }
    let _ = spring.unit_control().set_unit_neutral(unit_id, true);
    let _ = spring.unit_control().set_unit_blocking(
        unit_id,
        spring::unit_control::SetUnitBlockingOptions {
            blocking: false,
            solid_objects: false,
            projectiles: false,
            quad_map_rays: false,
            crushable: false,
            block_enemy_pushing: false,
            block_height_changes: false,
        },
    );
}

const HEIGHT: f32 = 1.0;
const AIM_HEIGHT: f32 = 25.0;
