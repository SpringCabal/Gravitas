use crate::state;

pub fn suppress() {
    let units = spring::get_all_units().unwrap_or_default();
    let known = state::with_state(|ui| ui.suppressed_units).unwrap_or(0);
    if units.len() as i32 == known {
        return;
    }
    for unit_id in &units {
        let _ = spring::unsynced_ctrl::set_unit_no_select(*unit_id, true);
        if is_effect(*unit_id) {
            let _ = spring::unsynced_ctrl::set_unit_no_draw(*unit_id, true);
        }
    }
    let _ = spring::selection::select_unit_array(&[], false);
    let _ = state::with_state(|ui| ui.suppressed_units = units.len() as i32);
}

fn is_effect(unit_id: i32) -> bool {
    let Ok(def_id) = spring::units_info::get_unit_def_id(unit_id) else {
        return false;
    };
    spring::unit_defs::get_unit_def_custom_param_keys(def_id)
        .unwrap_or_default()
        .iter()
        .any(|key| key == "effect")
}
