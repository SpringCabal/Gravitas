use spring::ResultExt;
use spring::commands;

use crate::state::Global;

pub fn perform(global: &Global) {
    global.restart.begin_wipe();
    reset(global);
    for unit_id in spring::get_all_units().unwrap_or_default() {
        commands::destroy_unit(
            unit_id,
            spring::generated::owned::unit_control::DestroyUnitOptions {
                selfd: false,
                reclaimed: true,
                attacker_id: -1,
                recycle_id: false,
            },
        )
        .then(|removed| {
            removed.log_err_once("restart: removing a unit");
        });
    }
    commands::run(|| {
        crate::with_global(|global| {
            global.restart.finish_wipe();
            reset(global);
        })
    });
}

fn reset(global: &Global) {
    let Global {
        level,
        player_unit,
        pending_rules: _,
        player_input,
        jumps,
        fall_immunity,
        impulses,
        plates,
        terrain,
        restart: _,
        outcome,
        tests: _,
    } = global;
    *level.access_mut() = Default::default();
    *jumps.access_mut() = Default::default();
    *fall_immunity.access_mut() = Default::default();
    *impulses.access_mut() = Default::default();
    *plates.access_mut() = Default::default();
    *terrain.access_mut() = Default::default();
    player_unit.set(None);
    player_input.set(None);
    outcome.set(None);
}
