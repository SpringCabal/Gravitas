use crate::units;

pub fn configure(unit_id: i32, unit_def_id: i32) {
    if !units::has_custom_param(unit_def_id, "wall") {
        return;
    }
    let _ = spring::set_unit_always_visible(unit_id, true);
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
    let _ = spring::set_unit_neutral(unit_id, true);
    let _ = spring::move_ctrl::move_ctrl(unit_id, true);
    let _ = spring::move_ctrl::set_no_blocking(unit_id, true);
    let _ = spring::move_ctrl::set_move_ctrl_gravity(unit_id, 0.0);
}
