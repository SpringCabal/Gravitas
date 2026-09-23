//! One-time level setup.

use alloc::format;

use crate::{diagnostics, level, mission, state, tests, units};

pub fn take_setup_flag() -> bool {
    state::with_world(|world| {
        let needs_setup = !world.initialized;
        world.initialized = true;
        needs_setup
    })
    .unwrap_or(false)
}

pub fn setup() {
    reveal_map();
    if tests::stage_level() {
        return;
    }
    let mut spawned = alloc::vec::Vec::with_capacity(level::SPAWNS.len());
    let mut failed = 0;
    for spawn in level::SPAWNS {
        match units::spawn(spawn) {
            Some(unit_id) => spawned.push((spawn.id, unit_id)),
            None => {
                failed += 1;
                diagnostics::warning(&format!(
                    "spawn failed: {} at {} {} (def {:?})",
                    spawn.def_name,
                    spawn.x,
                    spawn.z,
                    units::def_id(spawn.def_name)
                ));
            }
        }
    }
    let _ = state::with_world(|world| {
        world.spawned = spawned;
        world.pending_link = true;
    });
    let duplicates = state::with_world(|world| world.duplicate_creates).unwrap_or(0);
    diagnostics::info(&format!(
        "level spawned: {} of {} units, {} duplicate unit_created",
        level::SPAWNS.len() - failed,
        level::SPAWNS.len(),
        duplicates
    ));
}

fn link_gates(world: &mut state::World) {
    for link in level::LINKS {
        let plate = world.unit_of(link.plate);
        let gate = world.unit_of(link.gate);
        let linked = match (plate, gate) {
            (Some(plate), Some(gate)) => world.plates.simple_link(plate, gate),
            _ => false,
        };
        if !linked {
            diagnostics::warning(&format!(
                "plate link failed: plate {} gate {}",
                link.plate, link.gate
            ));
        }
    }
    world.plates.enable_link_checks();
    for link in level::BITMASK_LINKS {
        let Some(gate) = world.unit_of(link.gate) else {
            diagnostics::warning("bitmask link failed: missing gate");
            continue;
        };
        let mut mask = alloc::vec::Vec::new();
        for plate in link.positive {
            if let Some(unit_id) = world.unit_of(*plate) {
                mask.push((unit_id, true));
            }
        }
        for plate in link.negative {
            if let Some(unit_id) = world.unit_of(*plate) {
                mask.push((unit_id, false));
            }
        }
        if !world.plates.bitmask_link(mask, gate) {
            diagnostics::warning("bitmask link failed");
        }
    }
}

fn reveal_map() {
    for ally_team in 0..2 {
        let _ = spring::team_control::set_global_los(ally_team, true);
    }
}

/// Link plates to gates on the frame after the level spawned.
pub fn link() {
    let _ = state::with_world(|world| {
        if !world.pending_link {
            return;
        }
        world.pending_link = false;
        link_gates(world);
        mission::stun_electrafi_proximity(world);
    });
}
