use crate::state::{Global, Level};

/// Remove the level and everything in it; the level gadget spawns it again
/// on the next frame.
pub fn perform(global: &Global) {
    global.restart.begin_wipe();
    for unit_id in spring::get_all_units().unwrap_or_default() {
        let _ = spring::destroy_unit(
            unit_id,
            spring::generated::owned::unit_control::DestroyUnitOptions {
                selfd: false,
                reclaimed: true,
                attacker_id: -1,
                recycle_id: false,
            },
        );
    }
    global.restart.finish_wipe();
    *global.level.access_mut() = Level::default();
    *global.plates.access_mut() = Default::default();
    *global.jumps.access_mut() = Default::default();
    *global.fall_immunity.access_mut() = Default::default();
    *global.impulses.access_mut() = Default::default();
    *global.terrain.access_mut() = Default::default();
    global.player_input.set(None);
    global.player_unit.set(None);
    global.outcome.set(None);
}
