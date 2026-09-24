use alloc::string::String;

use springcabal::message::Encoder;

use crate::state::Global;
use crate::units;

/// What the HUD shows: the player unit, its health and which gun is loaded.
pub fn message(global: &Global, frame: i32) -> String {
    let player = global.player_unit.get().unwrap_or(-1);
    let (health, max_health) = health(player);
    Encoder::new("GRAVITAS_STATUS")
        .kv("playerUnit", player)
        .kv("health", health)
        .kv("maxHealth", max_health)
        .kv_flag("push", reload_ready(player, units::WEAPON_PUSH, frame))
        .kv_flag("pull", reload_ready(player, units::WEAPON_PULL, frame))
        .finish()
}

fn health(unit_id: i32) -> (i32, i32) {
    if unit_id < 0 {
        return (0, 1);
    }
    spring::units_info::get_unit_health(unit_id)
        .map(|health| (health.health.max(0.0) as i32, health.max_health as i32))
        .unwrap_or((0, 1))
}

fn reload_ready(unit_id: i32, weapon: i32, frame: i32) -> bool {
    if unit_id < 0 {
        return false;
    }
    spring::units_weapons::get_unit_weapon_state(unit_id, weapon, "reloadFrame")
        .is_ok_and(|state| state.reload_frame <= frame as f32)
}
