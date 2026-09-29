//! Level setup: spawn on one frame, link on the next.

use alloc::format;
use alloc::rc::Rc;
use alloc::vec::Vec;
use core::cell::RefCell;

use spring::commands;

use crate::state::{Global, Phase};
use crate::{diagnostics, level, mission, tests, units};

/// Spawn the level's units (or the e2e arena instead). The level is linked
/// on the next frame.
pub fn spawn(global: &Global) {
    reveal_map();
    if tests::stage_level(global) {
        global.level.access_mut().phase = Phase::Running;
        return;
    }
    let spawned = Rc::new(RefCell::new(Vec::with_capacity(level::SPAWNS.len())));
    for spawn in level::SPAWNS {
        let spawned = Rc::clone(&spawned);
        units::spawn(spawn).then(move |unit_id| match unit_id {
            Some(unit_id) => spawned.borrow_mut().push((spawn.id, unit_id)),
            None => spring::log_warning!(
                crate::diagnostics::LOG_SECTION,
                "spawn failed: {} at {} {} (def {:?})",
                spawn.def_name,
                spawn.x,
                spawn.z,
                crate::defs::get().id(spawn.def_name)
            ),
        });
    }
    // After every unit above was created.
    commands::run(move || {
        let spawned = spawned.take();
        crate::with_global(|global| {
            let mut level = global.level.access_mut();
            diagnostics::info(&format!(
                "level spawned: {} of {} units",
                spawned.len(),
                level::SPAWNS.len(),
            ));
            level.spawned = spawned;
            level.phase = Phase::Link;
        });
    });
}

/// Link plates to gates and stun the robots standing in electric fields, on
/// the frame after the level spawned.
pub fn link(global: &Global) {
    {
        let mut level = global.level.access_mut();
        level.phase = Phase::Running;
        let mut plates = global.plates.access_mut();
        level.link_failures = link_gates(&level, &mut plates);
    }
    mission::stun_electrafi_proximity();
}

/// Returns how many links failed.
fn link_gates(level: &crate::state::Level, plates: &mut crate::plate::Plates) -> u32 {
    let mut failures = 0;
    for link in level::LINKS {
        let plate = level.unit_of(link.plate);
        let gate = level.unit_of(link.gate);
        let linked = match (plate, gate) {
            (Some(plate), Some(gate)) => plates.simple_link(plate, gate),
            _ => false,
        };
        if !linked {
            failures += 1;
            spring::log_warning!(
                crate::diagnostics::LOG_SECTION,
                "plate link failed: plate {} gate {}",
                link.plate,
                link.gate
            );
        }
    }
    plates.enable_link_checks();
    for link in level::BITMASK_LINKS {
        let Some(gate) = level.unit_of(link.gate) else {
            failures += 1;
            spring::log_warning!(
                crate::diagnostics::LOG_SECTION,
                "bitmask link failed: missing gate"
            );
            continue;
        };
        let mut mask = Vec::new();
        for plate in link.positive {
            if let Some(unit_id) = level.unit_of(*plate) {
                mask.push((unit_id, true));
            }
        }
        for plate in link.negative {
            if let Some(unit_id) = level.unit_of(*plate) {
                mask.push((unit_id, false));
            }
        }
        if !plates.bitmask_link(mask, gate) {
            failures += 1;
            spring::log_warning!(crate::diagnostics::LOG_SECTION, "bitmask link failed");
        }
    }
    failures
}

fn reveal_map() {
    for ally_team in 0..2 {
        let _ = spring::team_control::set_global_los(ally_team, true);
    }
}
