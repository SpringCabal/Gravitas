use gravitas_shared::{Effect, EffectKind};
use spring::ResultExt;

use crate::units;

pub fn tag_on_create(
    spring: &spring::Spring<'_, spring::RulesSynced>,
    unit_id: i32,
    unit_def_id: i32,
) {
    let def = crate::defs::get(spring).unit(unit_def_id);
    if def.invulnerable {
        set_rules_param(spring, unit_id, INVULNERABLE, 1.0);
    }
    if def.nonparalyzable {
        set_rules_param(spring, unit_id, NONPARALYZABLE, 1.0);
    }
}

pub fn mark_respawnable(spring: &spring::Spring<'_, spring::RulesSynced>, unit_id: i32) {
    set_rules_param(spring, unit_id, RESPAWNABLE, 1.0);
}

pub fn is_respawnable(spring: &spring::Spring<'_, spring::RulesSynced>, unit_id: i32) -> bool {
    rules_flag(spring, unit_id, RESPAWNABLE)
}

const RESPAWNABLE: &str = "respawnable";
pub const INVULNERABLE: &str = "invulnerable";
const NONPARALYZABLE: &str = "nonparalyzable";

pub fn is_invulnerable(spring: &spring::Spring<'_, spring::RulesSynced>, unit_id: i32) -> bool {
    rules_flag(spring, unit_id, INVULNERABLE)
}

pub fn blocks_damage(
    spring: &spring::Spring<'_, spring::RulesSynced>,
    unit_id: i32,
    paralyzer: bool,
) -> bool {
    if paralyzer {
        rules_flag(spring, unit_id, NONPARALYZABLE)
    } else {
        is_invulnerable(spring, unit_id)
    }
}

pub fn explode_barrel(
    spring: &spring::Spring<'_, spring::RulesSynced>,
    barrel: i32,
    position: spring::Float3,
) {
    let _ = spring.sound().play_sound_file(
        EXPLOSION_SOUND,
        EXPLOSION_VOLUME,
        spring::Float3::default(),
        spring::Float3::default(),
        0,
    );
    let _ = spring.effects_control().spawn_ceg(
        &spring::effects_control::DefRef {
            name: EXPLOSION_CEG.into(),
            id: -1,
        },
        position,
        spring::Float3::default(),
        0.0,
        0.0,
        0.0,
    );
    send_effect(
        spring,
        position,
        EffectKind::Flame {
            radius: EXPLOSION_RADIUS,
        },
    );
    for nearby in units::units_in_cylinder(spring, position.x, position.z, EXPLOSION_RADIUS) {
        if !units::valid(spring, nearby) {
            continue;
        }
        spring
            .unit_control()
            .add_unit_damage(
                nearby,
                EXPLOSION_DAMAGE,
                0.0,
                -1,
                barrel,
                spring::Float3::default(),
            )
            .log_err_once(spring, "barrel blast damage");
    }
}

pub fn kill_fallen(spring: &spring::Spring<'_, spring::RulesSynced>, frame: i32) {
    if frame % FALL_CHECK_RATE != 0 {
        return;
    }
    let Ok(map) = spring.game().get_game_map_info() else {
        return;
    };
    let low = spring::Float3 {
        x: 0.0,
        y: FALL_FLOOR,
        z: 0.0,
    };
    let high = spring::Float3 {
        x: map.map_size_x as f32,
        y: FALL_CEILING,
        z: map.map_size_z as f32,
    };
    for unit_id in units::units_in_box(spring, low, high) {
        crate::with_addon_context(|ctx| {
            ctx.commands().run(move |spring| {
                if !units::valid(spring, unit_id) {
                    return;
                }
                spring
                    .unit_control()
                    .destroy_unit(
                        unit_id,
                        spring::unit_control::DestroyUnitOptions {
                            selfd: false,
                            reclaimed: true,
                            attacker_id: -1,
                            recycle_id: false,
                        },
                    )
                    .log_err_once(spring, "removing a unit that fell off the map");
            })
        });
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

fn set_rules_param(
    spring: &spring::Spring<'_, spring::RulesSynced>,
    unit_id: i32,
    name: &str,
    value: f32,
) {
    if spring
        .rules_params()
        .set_unit_rules_param_f32(unit_id, name, value, units::RULES_PARAM_PRIVATE)
        .is_err()
    {
        spring::log_warning!(
            spring,
            crate::diagnostics::LOG_SECTION,
            "set_unit_rules_param_f32 failed for {name} on unit {unit_id}"
        );
    }
}

fn rules_flag(spring: &spring::Spring<'_, spring::RulesSynced>, unit_id: i32, name: &str) -> bool {
    spring
        .rules_params()
        .get_unit_rules_param_f32(unit_id, name)
        .unwrap_or(0.0)
        >= 1.0
}

pub fn send_effect(
    spring: &spring::Spring<'_, spring::RulesSynced>,
    position: spring::Float3,
    kind: EffectKind,
) {
    let effect = Effect {
        position: [position.x, position.y, position.z],
        kind,
    };
    springcabal::message::send_to_ui(spring, &effect.encode());
}
