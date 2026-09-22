//! Unit call-in bodies: creation, destruction and damage.

use crate::orchestration::move_away;
use crate::robots::Robots;
use crate::{
    effects, falldamage, level, mission, respawn, restart, shapes, state, terrain, tests, units,
    walls,
};

pub fn unit_created(unit_id: i32, unit_def_id: i32, unit_team: i32, _builder_id: i32) {
    if state::with_world(|world| world.observe_created(unit_id)) != Some(true) {
        return;
    }
    let _ = state::with_world(|world| {
        world.player.adopt(unit_id, unit_def_id, unit_team);
        world.plates.created(unit_id, unit_def_id);
        world.electrafi.created(unit_id, unit_def_id);
    });
    effects::tag_on_create(unit_id, unit_def_id);
    walls::configure(unit_id, unit_def_id);
    shapes::apply(unit_id, unit_def_id);
    shapes::configure_effect(unit_id, unit_def_id);
    terrain::defer_wall_terrain(unit_id, unit_def_id, 1.0);
    let _ = state::with_world(|world| {
        let frame = world.frame;
        world.fires.created(unit_id, unit_def_id);
        if effects::is_respawnable(unit_id) {
            world.respawns.adopt(unit_id, unit_def_id, unit_team, frame);
        }
    });
}

pub fn unit_destroyed(
    unit_id: i32,
    unit_def_id: i32,
    _unit_team: i32,
    _attacker_id: i32,
    _attacker_def_id: i32,
    _attacker_team: i32,
    _weapon_def_id: i32,
) {
    tests::note_destroyed(unit_id, unit_def_id);
    let _ = state::with_world(|world| world.forget_created(unit_id));
    if !restart::wiping() {
        effects::explode_barrel(unit_id, unit_def_id);
    }
    terrain::defer_wall_terrain(unit_id, unit_def_id, 0.0);
    report_game_over(unit_id, unit_def_id);
    let rebirth = state::with_world(|world| world.respawns.destroyed(unit_id)).flatten();
    if let Some(rebirth) = rebirth {
        respawn::rebirth(&rebirth);
    }
    let _ = state::with_world(|world| {
        world.player.release(unit_id);
        world.plates.destroyed(unit_id);
        world.fall_damage.forget(unit_id);
        world.jumps.forget(unit_id);
        world.electrafi.destroyed(unit_id);
        world.fires.destroyed(unit_id);
    });
}

pub fn unit_pre_damaged(
    unit_id: i32,
    unit_def_id: i32,
    unit_team: i32,
    damage: f32,
    paralyzer: bool,
    weapon_def_id: i32,
    _projectile_id: i32,
    attacker_id: i32,
    attacker_def_id: i32,
    attacker_team: i32,
) -> spring::DamageResult {
    let frame = state::with_world(|world| world.frame).unwrap_or(0);
    let _ = state::with_world(|world| {
        world
            .impulses
            .damaged(unit_id, unit_def_id, weapon_def_id, attacker_id)
    });
    let avoidance = state::with_world(|world| {
        world
            .robots
            .flee(unit_id, unit_def_id, attacker_id, attacker_def_id, frame)
    })
    .flatten();
    if let Some(avoidance) = avoidance {
        move_away(avoidance);
    }
    let mut damage = damage;
    if Robots::is_friendly_fire(unit_def_id, attacker_def_id, unit_team == attacker_team) {
        damage = 0.0;
    }
    if weapon_def_id == falldamage::OBJECT_COLLISION {
        damage = 0.0;
    } else if weapon_def_id == falldamage::GROUND_COLLISION {
        let immune =
            state::with_world(|world| world.fall_damage.is_immune(unit_id, frame)).unwrap_or(false);
        damage = if immune {
            0.0
        } else {
            falldamage::ground_collision(unit_id, unit_def_id)
        };
        play_bounce(unit_id, damage);
    }
    if effects::blocks_damage(unit_id, paralyzer) {
        damage = 0.0;
    }
    spring::DamageResult::unchanged(damage)
}

fn play_bounce(unit_id: i32, damage: f32) {
    if damage <= BOUNCE_DAMAGE_THRESHOLD {
        return;
    }
    let Some(position) = units::position(unit_id) else {
        return;
    };
    if spring::units_info::get_unit_health(unit_id).is_ok_and(|health| health.health <= 0.0) {
        return;
    }
    let _ = spring::sound::play_sound_file(
        BOUNCE_SOUND,
        BOUNCE_VOLUME,
        position,
        spring::types::Float3::default(),
        0,
    );
}

const BOUNCE_DAMAGE_THRESHOLD: f32 = 10.0;
const BOUNCE_SOUND: &str = "sounds/bounce.ogg";
const BOUNCE_VOLUME: f32 = 20.0;

fn report_game_over(unit_id: i32, unit_def_id: i32) {
    if restart::wiping() {
        return;
    }
    if units::def_id(units::GRAVIT) == Some(unit_def_id) {
        mission::win(&[ENEMY_ALLY_TEAM]);
        return;
    }
    let boss = state::with_world(|world| world.unit_of(level::BOSS_UNIT)).flatten();
    if boss == Some(unit_id) {
        mission::win(&[PLAYER_ALLY_TEAM]);
    }
}

const PLAYER_ALLY_TEAM: i32 = 0;
const ENEMY_ALLY_TEAM: i32 = 1;
