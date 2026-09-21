use alloc::format;

use crate::diagnostics;
use crate::state::World;

pub fn report(world: &World) {
    let _ = world;
    report_weapon_params();
}

pub fn report_late(world: &World, frame: i32) {
    let _ = world;
    if frame != SETTLE_FRAME {
        return;
    }
    diagnostics::info(&format!(
        "gravitas: terrain levelled {}",
        crate::terrain::levelled_report()
    ));
}

const SETTLE_FRAME: i32 = 30;

fn report_weapon_params() {
    for name in ["gravity_pos", "gravity_neg"] {
        let Ok(weapon_def_id) = spring::weapon_defs::get_weapon_def_id(name) else {
            continue;
        };
        if spring::weapon_defs::get_weapon_def_custom_param(weapon_def_id, "impulse").is_err() {
            diagnostics::warning(&format!(
                "gravitas: get_weapon_def_custom_param(impulse) missing for {name}, using the baked table"
            ));
        }
    }
}
