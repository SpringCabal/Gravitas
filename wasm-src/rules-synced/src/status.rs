use alloc::format;

use crate::state::Global;
use crate::units;

/// Send the HUD what it shows: player health and which gun is loaded.
pub fn publish(global: &Global, frame: i32) {
    let player = global.player_unit.get().unwrap_or(-1);
    let (health, max_health) = health(player);
    let message = format!(
        "GRAVITAS_STATUS|frame={frame}|playerUnit={player}|health={health}|maxHealth={max_health}|push={}|pull={}",
        reload_ready(player, units::WEAPON_PUSH, frame),
        reload_ready(player, units::WEAPON_PULL, frame),
    );
    let _ = spring::send_to_unsynced(&message);
}

fn health(unit_id: i32) -> (i32, i32) {
    if unit_id < 0 {
        return (0, 1);
    }
    spring::units_info::get_unit_health(unit_id)
        .map(|health| (health.health.max(0.0) as i32, health.max_health as i32))
        .unwrap_or((0, 1))
}

fn reload_ready(unit_id: i32, weapon: i32, frame: i32) -> i32 {
    if unit_id < 0 {
        return 0;
    }
    spring::units_weapons::get_unit_weapon_state(unit_id, weapon, "reloadFrame")
        .map(|state| i32::from(state.reload_frame <= frame as f32))
        .unwrap_or(0)
}
