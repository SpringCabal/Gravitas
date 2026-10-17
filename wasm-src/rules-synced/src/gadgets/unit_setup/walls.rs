pub fn configure(spring: &spring::Spring<'_, spring::RulesSynced>, unit_id: i32, unit_def_id: i32) {
    if !crate::defs::get(spring).unit(unit_def_id).wall {
        return;
    }
    let _ = spring.unit_control().set_unit_always_visible(unit_id, true);
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
    let _ = spring.unit_control().set_unit_neutral(unit_id, true);
    crate::units::move_ctrl(spring, unit_id, true);
    let _ = spring.move_ctrl().set_no_blocking(unit_id, true);
    let _ = spring.move_ctrl().set_move_ctrl_gravity(unit_id, 0.0);
}
