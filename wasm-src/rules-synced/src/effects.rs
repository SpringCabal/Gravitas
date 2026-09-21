use crate::units;

pub fn tag_on_create(unit_id: i32, unit_def_id: i32) {
    set_flag(unit_id, unit_def_id, "invulnerable");
    set_flag(unit_id, unit_def_id, "nonparalyzable");
    if units::custom_param(unit_def_id, "respawnable").is_some() {
        set_rules_param(unit_id, "respawnable", 1.0);
    }
}

pub fn is_respawnable(unit_id: i32) -> bool {
    rules_flag(unit_id, "respawnable")
}

pub fn blocks_damage(unit_id: i32, paralyzer: bool) -> bool {
    if !paralyzer && rules_flag(unit_id, "invulnerable") {
        return true;
    }
    paralyzer && rules_flag(unit_id, "nonparalyzable")
}

pub fn explode_barrel(unit_id: i32, unit_def_id: i32) {
    if units::custom_param(unit_def_id, "barrel").is_none() {
        return;
    }
    let Some(position) = units::position(unit_id) else {
        return;
    };
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
    let _ = spring::send_to_unsynced(&alloc::format!(
        "GRAVITAS_FX|flame|{}|{}|{}|{EXPLOSION_RADIUS}",
        position.x,
        position.y,
        position.z
    ));
    for nearby in units::units_in_cylinder(position.x, position.z, EXPLOSION_RADIUS) {
        let _ = spring::add_unit_damage(
            nearby,
            EXPLOSION_DAMAGE,
            0.0,
            -1,
            unit_id,
            spring::types::Float3::default(),
        );
    }
}

pub fn kill_fallen(frame: i32) {
    if frame % FALL_CHECK_RATE != 0 {
        return;
    }
    let Ok(map) = spring::game::get_game_map_info(0) else {
        return;
    };
    let mut output = alloc::vec![0i32; 256];
    let found = match spring::get_units_in_box_into(
        spring::types::Float3 {
            x: 0.0,
            y: FALL_FLOOR,
            z: 0.0,
        },
        spring::types::Float3 {
            x: map.map_size_x as f32,
            y: FALL_CEILING,
            z: map.map_size_z as f32,
        },
        ANY_ALLEGIANCE,
        &mut output,
    ) {
        Ok(spring::BufferFill::Complete(count)) => count,
        _ => return,
    };
    output.truncate(found);
    for unit_id in output {
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
}

const EXPLOSION_RADIUS: f32 = 250.0;
const EXPLOSION_DAMAGE: f32 = 500.0;
const EXPLOSION_SOUND: &str = "sounds/explosion.ogg";
const EXPLOSION_VOLUME: f32 = 20.0;
const EXPLOSION_CEG: &str = "flashnuke";
const FALL_CHECK_RATE: i32 = 31;
const FALL_FLOOR: f32 = -500.0;
const FALL_CEILING: f32 = 50.0;
const ANY_ALLEGIANCE: i32 = -1;

fn set_flag(unit_id: i32, unit_def_id: i32, key: &str) {
    if units::custom_param_is_one(unit_def_id, key) {
        set_rules_param(unit_id, key, 1.0);
    }
}

fn set_rules_param(unit_id: i32, name: &str, value: f32) {
    if spring::set_unit_rules_param_f32(unit_id, name, value, units::RULES_PARAM_PRIVATE).is_err() {
        crate::diagnostics::warning(&alloc::format!(
            "set_unit_rules_param_f32 failed for {name} on unit {unit_id}"
        ));
    }
}

fn rules_flag(unit_id: i32, name: &str) -> bool {
    spring::get_unit_rules_param_f32(unit_id, name).unwrap_or(0.0) >= 1.0
}
