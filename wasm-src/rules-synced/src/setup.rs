//! Level setup: spawn on one frame, link on the next.

use alloc::format;
use alloc::vec::Vec;

use crate::state::{Global, Phase};
use crate::{diagnostics, level, mission, tests, units};

/// Spawn the level's units (or the e2e arena instead).
pub fn spawn(global: &Global) {
    reveal_map();
    if tests::stage_level(global) {
        global.level.access_mut().phase = Phase::Running;
        return;
    }
    let mut spawned = Vec::with_capacity(level::SPAWNS.len());
    let mut failed = 0;
    for spawn in level::SPAWNS {
        match units::spawn(spawn, &global.pending_rules) {
            Some(unit_id) => spawned.push((spawn.id, unit_id)),
            None => {
                failed += 1;
                diagnostics::warning(&format!(
                    "spawn failed: {} at {} {} (def {:?})",
                    spawn.def_name,
                    spawn.x,
                    spawn.z,
                    crate::defs::get().id(spawn.def_name)
                ));
            }
        }
    }
    let mut level = global.level.access_mut();
    level.spawned = spawned;
    level.phase = Phase::Link;
    diagnostics::info(&format!(
        "level spawned: {} of {} units",
        level::SPAWNS.len() - failed,
        level::SPAWNS.len(),
    ));
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
            diagnostics::warning(&format!(
                "plate link failed: plate {} gate {}",
                link.plate, link.gate
            ));
        }
    }
    plates.enable_link_checks();
    for link in level::BITMASK_LINKS {
        let Some(gate) = level.unit_of(link.gate) else {
            failures += 1;
            diagnostics::warning("bitmask link failed: missing gate");
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
            diagnostics::warning("bitmask link failed");
        }
    }
    failures
}

fn reveal_map() {
    for ally_team in 0..2 {
        let _ = spring::team_control::set_global_los(ally_team, true);
    }
}
