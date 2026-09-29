use spring::ResultExt;
use spring::commands;

use crate::state::Global;

/// Remove the level and everything in it; the level gadget spawns it again
/// on the next frame.
pub fn perform(global: &Global) {
    global.restart.begin_wipe();
    // Now, so the gadgets that run after this one this frame find no level,
    // as when the units were gone before them.
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
    // After the removed units' UnitDestroyed, which see `wiping()` and record
    // their deaths into the level state: drop that too.
    commands::run(|| {
        crate::with_global(|global| {
            global.restart.finish_wipe();
            reset(global);
        })
    });
}

/// Put every part of `Global` that belongs to a level back to its default.
/// The pattern names every field, so a new one has to be sorted here.
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
        // Counts restarts; gadget state in `PerLevel` resets from it.
        restart: _,
        outcome,
        // The e2e cases span restarts.
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
