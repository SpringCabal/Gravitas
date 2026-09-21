//! The per-frame steps the gadgets drive, and the unit call-in bodies.
//!
//! These were the private helpers of the original `callins` module.

use alloc::format;

use crate::robots::Avoidance;
use crate::{diagnostics, jump, mission, plate, respawn, state, terrain, tests, units};

pub fn start_jump() {
    let request = state::with_world(|world| world.player.take_jump_request()).flatten();
    let Some((unit_id, x, y, z)) = request else {
        return;
    };
    let outcome = state::with_world(|world| world.jumps.start(unit_id, x, y, z));
    match outcome {
        Some(jump::Outcome::OutOfRange) => {
            report_jump(None);
            let _ = state::with_world(|world| world.player.retry_jump(x, y, z));
        }
        Some(jump::Outcome::Started) | None => report_jump(None),
        Some(outcome) => report_jump(Some(outcome.reason())),
    }
}

fn report_jump(reason: Option<&'static str>) {
    LAST_JUMP_REFUSAL.with(|last| {
        if last.get() == reason {
            return;
        }
        last.set(reason);
        if let Some(reason) = reason {
            diagnostics::info(&format!("jump refused: {reason}"));
        }
    });
}

thread_local! {
    static LAST_JUMP_REFUSAL: core::cell::Cell<Option<&'static str>> =
        const { core::cell::Cell::new(None) };
}

pub fn land_jumps(frame: i32) {
    let landed = state::with_world(|world| world.jumps.update()).unwrap_or_default();
    for unit_id in landed {
        let _ = state::with_world(|world| {
            world
                .fall_damage
                .set_immunity(unit_id, frame + jump::LANDING_IMMUNITY_FRAMES)
        });
    }
}

pub fn apply_electrafi(frame: i32) {
    let shocks = state::with_world(|world| world.electrafi.update(frame)).unwrap_or_default();
    for shock in shocks {
        let paralyze_time = if shock.paralyze { PARALYZING } else { 0.0 };
        let _ = spring::add_unit_damage(
            shock.unit_id,
            shock.damage,
            paralyze_time,
            -1,
            shock.attacker_id,
            spring::types::Float3::default(),
        );
        emit_spark(shock.unit_id, shock.intensity);
    }
    let arcs = state::with_world(|world| world.electrafi.arcs(frame)).unwrap_or_default();
    for arc in arcs {
        let _ = spring::send_to_unsynced(&alloc::format!(
            "GRAVITAS_FX|arc|{}|{}|{}|{}",
            arc.position.x,
            arc.position.y,
            arc.position.z,
            arc.radius
        ));
    }
}

pub fn move_away(avoidance: Avoidance) {
    if avoidance.replace_order {
        let _ = spring::give_order_to_unit(avoidance.unit_id, CMD_STOP, &[], 0, 0);
    }
    let _ = spring::give_order_to_unit(
        avoidance.unit_id,
        CMD_MOVE,
        &[avoidance.goal.x, avoidance.goal.y, avoidance.goal.z],
        0,
        0,
    );
}

const CMD_STOP: i32 = 0;
const CMD_MOVE: i32 = 10;
const PARALYZING: f32 = 1.0;

pub fn apply_mission(frame: i32) {
    let assaults = state::with_world(|world| {
        let player_unit = world.player.unit_id();
        world.mission.update(player_unit, frame)
    })
    .unwrap_or_default();
    for assault in &assaults {
        mission::issue_assault(assault);
    }
}

pub fn apply_respawns(frame: i32) {
    if frame == respawn::SCAN_FRAME {
        let _ = state::with_world(|world| world.respawns.scan(frame));
    }
    let rebirths = state::with_world(|world| world.respawns.update(frame)).unwrap_or_default();
    for entry in &rebirths {
        respawn::rebirth(entry);
    }
}

fn emit_spark(unit_id: i32, intensity: f32) {
    let Some(position) = crate::units::position(unit_id) else {
        return;
    };
    let velocity = spring::units_info::get_unit_velocity(unit_id).unwrap_or_default();
    let _ = spring::send_to_unsynced(&alloc::format!(
        "GRAVITAS_FX|spark|{}|{}|{}|{intensity}|{}|{}",
        position.x,
        position.y,
        position.z,
        velocity.x,
        velocity.z
    ));
}

pub fn apply_fire(frame: i32) {
    let burns = state::with_world(|world| world.fires.update(frame)).unwrap_or_default();
    for burn in burns {
        let _ = spring::add_unit_damage(
            burn.unit_id,
            burn.damage,
            0.0,
            -1,
            burn.attacker_id,
            spring::types::Float3::default(),
        );
        emit_burn(burn.unit_id, burn.size_mult);
    }
    let _ = state::with_world(|world| world.fires.take_extinguished());
    let flames = state::with_world(|world| world.fires.flames(frame)).unwrap_or_default();
    for flame in flames {
        let _ = spring::send_to_unsynced(&alloc::format!(
            "GRAVITAS_FX|flame|{}|{}|{}|{}",
            flame.position.x,
            flame.position.y,
            flame.position.z,
            flame.radius
        ));
    }
}

fn emit_burn(unit_id: i32, size_mult: f32) {
    let Some(position) = units::position(unit_id) else {
        return;
    };
    let radius = spring::units_info::get_unit_radius(unit_id).unwrap_or(DEFAULT_BURN_RADIUS);
    let velocity = spring::units_info::get_unit_velocity(unit_id).unwrap_or_default();
    let _ = spring::send_to_unsynced(&alloc::format!(
        "GRAVITAS_FX|burn|{}|{}|{}|{radius}|{size_mult}|{}|{}",
        position.x,
        position.y,
        position.z,
        velocity.x,
        velocity.z
    ));
}

const DEFAULT_BURN_RADIUS: f32 = 20.0;

pub fn apply_plate_changes(frame: i32) {
    let changes = state::with_world(|world| world.plates.pending(frame)).unwrap_or_default();
    for (unit_id, active) in changes {
        tests::note_toggled(unit_id, active, frame);
        plate::set_state(unit_id, active);
        adjust_gate_terrain(unit_id, active, frame);
    }
}

fn adjust_gate_terrain(unit_id: i32, active: bool, frame: i32) {
    let Some(unit_def_id) = units::def_of(unit_id) else {
        return;
    };
    if units::def_id(GATE) != Some(unit_def_id) {
        return;
    }
    if active {
        terrain::defer_wall_terrain(unit_id, unit_def_id, 0.0);
        return;
    }
    terrain::defer_wall_terrain_until(unit_id, unit_def_id, 1.0, frame + GATE_CLOSE_FRAMES);
}

const GATE: &str = "gate";
const GATE_CLOSE_FRAMES: i32 = 27;
