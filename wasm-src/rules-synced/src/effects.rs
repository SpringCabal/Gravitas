use gravitas_shared::{Effect, EffectKind};
use spring::ResultExt;

use crate::units;

pub fn tag_on_create(unit_id: i32, unit_def_id: i32) {
    let def = crate::defs::get().unit(unit_def_id);
    if def.invulnerable {
        set_rules_param(unit_id, INVULNERABLE, 1.0);
    }
    if def.nonparalyzable {
        set_rules_param(unit_id, NONPARALYZABLE, 1.0);
    }
}

/// Respawnable units come back when they die or get stuck. No def is; the
/// scenario marks units with this rules param.
pub fn mark_respawnable(unit_id: i32) {
    set_rules_param(unit_id, RESPAWNABLE, 1.0);
}

pub fn is_respawnable(unit_id: i32) -> bool {
    rules_flag(unit_id, RESPAWNABLE)
}

const RESPAWNABLE: &str = "respawnable";
/// Rules params: the def's flags, which the scenario can override per unit.
pub const INVULNERABLE: &str = "invulnerable";
const NONPARALYZABLE: &str = "nonparalyzable";

pub fn is_invulnerable(unit_id: i32) -> bool {
    rules_flag(unit_id, INVULNERABLE)
}

pub fn blocks_damage(unit_id: i32, paralyzer: bool) -> bool {
    if paralyzer {
        rules_flag(unit_id, NONPARALYZABLE)
    } else {
        is_invulnerable(unit_id)
    }
}

/// The blast of a barrel that was destroyed at `position`.
pub fn explode_barrel(barrel: i32, position: spring::types::Float3) {
    let _ = spring::sound::play_sound_file(
        EXPLOSION_SOUND,
        EXPLOSION_VOLUME,
        spring::types::Float3::default(),
        spring::types::Float3::default(),
        0,
    );
    let _ = spring::spawn_ceg(
        EXPLOSION_CEG,
        position,
        spring::types::Float3::default(),
        0.0,
        0.0,
        0.0,
    );
    send_effect(
        position,
        EffectKind::Flame {
            radius: EXPLOSION_RADIUS,
        },
    );
    for nearby in units::units_in_cylinder(position.x, position.z, EXPLOSION_RADIUS) {
        // An earlier hit may have set off another barrel that killed it.
        if !units::valid(nearby) {
            continue;
        }
        spring::add_unit_damage(
            nearby,
            EXPLOSION_DAMAGE,
            0.0,
            -1,
            barrel,
            spring::types::Float3::default(),
        )
        .log_err_once("barrel blast damage");
    }
}

pub fn kill_fallen(frame: i32) {
    if frame % FALL_CHECK_RATE != 0 {
        return;
    }
    let Ok(map) = spring::game::get_game_map_info(0) else {
        return;
    };
    let low = spring::types::Float3 {
        x: 0.0,
        y: FALL_FLOOR,
        z: 0.0,
    };
    let high = spring::types::Float3 {
        x: map.map_size_x as f32,
        y: FALL_CEILING,
        z: map.map_size_z as f32,
    };
    for unit_id in units::units_in_box(low, high) {
        if !units::valid(unit_id) {
            continue;
        }
        spring::destroy_unit(
            unit_id,
            spring::generated::owned::unit_control::DestroyUnitOptions {
                selfd: false,
                reclaimed: true,
                attacker_id: -1,
                recycle_id: false,
            },
        )
        .log_err_once("removing a unit that fell off the map");
    }
}

const EXPLOSION_RADIUS: f32 = 250.0;
const EXPLOSION_DAMAGE: f32 = 500.0;
const EXPLOSION_SOUND: &str = "sounds/explosion.ogg";
const EXPLOSION_VOLUME: f32 = 20.0;
const EXPLOSION_CEG: &str = "flashnuke";
const FALL_CHECK_RATE: i32 = 31;
const FALL_FLOOR: f32 = -500.0;
const FALL_CEILING: f32 = 50.0;

fn set_rules_param(unit_id: i32, name: &str, value: f32) {
    if spring::set_unit_rules_param_f32(unit_id, name, value, units::RULES_PARAM_PRIVATE).is_err() {
        spring::log_warning!(
            crate::diagnostics::LOG_SECTION,
            "set_unit_rules_param_f32 failed for {name} on unit {unit_id}"
        );
    }
}

fn rules_flag(unit_id: i32, name: &str) -> bool {
    spring::get_unit_rules_param_f32(unit_id, name).unwrap_or(0.0) >= 1.0
}

/// Ask the ui module to draw a fire or electric effect at `position`.
pub fn send_effect(position: spring::types::Float3, kind: EffectKind) {
    let effect = Effect {
        position: [position.x, position.y, position.z],
        kind,
    };
    springcabal::message::send_to_ui(&effect.encode());
}
