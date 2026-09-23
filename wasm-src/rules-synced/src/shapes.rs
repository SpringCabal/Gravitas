pub fn apply(unit_id: i32, unit_def_id: i32) {
    let def = crate::defs::get().unit(unit_def_id);
    if let Some(radius) = def.radius {
        let _ = spring::set_unit_radius_and_height(unit_id, radius, HEIGHT);
    }
    let offset = spring::types::Float3 {
        x: 0.0,
        y: AIM_HEIGHT,
        z: 0.0,
    };
    let _ = spring::set_unit_mid_and_aim_pos(unit_id, offset, offset, true);
}

pub fn configure_effect(unit_id: i32, unit_def_id: i32) {
    if !crate::defs::get().unit(unit_def_id).effect {
        return;
    }
    let _ = spring::set_unit_neutral(unit_id, true);
    let _ = spring::set_unit_blocking(
        unit_id,
        spring::generated::owned::unit_control::SetUnitBlockingOptions {
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
