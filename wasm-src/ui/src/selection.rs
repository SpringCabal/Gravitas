/// Nothing in Gravitas is selectable, and effect units (fire, electric fields)
/// are drawn by the particle widget instead of as models.
pub fn configure(unit_id: i32, unit_def_id: i32) {
    let _ = spring::unsynced_ctrl::set_unit_no_select(unit_id, true);
    if crate::defs::unit(unit_def_id).effect {
        let _ = spring::unsynced_ctrl::set_unit_no_draw(unit_id, true);
    }
    let _ = spring::selection::select_unit_array(&[], false);
}

/// Configure the units that exist before the widget does.
pub fn configure_all() {
    for unit_id in spring::get_all_units().unwrap_or_default() {
        if let Ok(unit_def_id) = spring::units_info::get_unit_def_id(unit_id) {
            configure(unit_id, unit_def_id);
        }
    }
}
