use alloc::vec::Vec;

use spring::types::Float3;

use crate::units;

#[derive(Default)]
pub struct Robots {
    moving_away: Vec<(i32, i32)>,
}

pub struct Avoidance {
    pub unit_id: i32,
    pub goal: Float3,
    pub replace_order: bool,
}

impl Robots {
    pub fn flee(
        &mut self,
        unit_id: i32,
        unit_def_id: i32,
        attacker_id: i32,
        attacker_def_id: i32,
        frame: i32,
    ) -> Option<Avoidance> {
        if !is_robot(unit_def_id) || !is_inanimate(attacker_def_id) {
            return None;
        }
        if is_paralyzed(unit_id) || self.is_busy(unit_id, frame) {
            return None;
        }
        let (position, attacker) = (units::position(unit_id)?, units::position(attacker_id)?);
        let attack_radius = inanimate_radius(attacker_id, attacker_def_id);
        let dx = position.x - attacker.x;
        let dy = position.y - attacker.y;
        let dz = position.z - attacker.z;
        let distance = (dx * dx + dy * dy + dz * dz).sqrt();
        let search_radius = attack_radius.max(distance) * SEARCH_SCALE;
        let hazards = hazards_near(position, search_radius);
        let velocity = spring::units_info::get_unit_velocity(unit_id).unwrap_or_default();
        let goal = safest_location(position, &hazards, velocity)?;
        let replace_order = self.is_tracked(unit_id);
        Some(Avoidance {
            unit_id,
            goal,
            replace_order,
        })
    }

    /// Remember that `unit_id` is on its way out since `frame`.
    pub fn mark(&mut self, unit_id: i32, frame: i32) {
        if let Some(entry) = self.moving_away.iter_mut().find(|(id, _)| *id == unit_id) {
            entry.1 = frame;
            return;
        }
        self.moving_away.push((unit_id, frame));
    }

    pub fn command_done(&mut self, unit_id: i32) {
        self.moving_away.retain(|(id, _)| *id != unit_id);
    }
}

pub fn is_friendly_fire(unit_def_id: i32, attacker_def_id: i32, same_team: bool) -> bool {
    same_team && is_robot(unit_def_id) && is_robot(attacker_def_id)
}

/// Walk out of the way before carrying on with the queue (an assault's FIGHT
/// order stays behind the escape). A robot still on its previous escape drops
/// that one first, as `unit_robots.lua` does.
pub fn move_away(avoidance: Avoidance) {
    let unit_id = avoidance.unit_id;
    if avoidance.replace_order
        && let Some(previous) = spring::get_unit_commands(unit_id, 1)
            .ok()
            .and_then(|queue| queue.into_iter().next())
    {
        units::order(unit_id, spring::cmd::REMOVE, &[previous.tag as f32]);
    }
    let goal = avoidance.goal;
    // INSERT with ALT: the first parameter is a queue position, not a tag.
    units::order_with(
        unit_id,
        spring::cmd::INSERT,
        &[
            FRONT_OF_QUEUE,
            spring::cmd::MOVE as f32,
            NO_OPTIONS,
            goal.x,
            goal.y,
            goal.z,
        ],
        spring::cmd::options::ALT_KEY,
    );
}

const FRONT_OF_QUEUE: f32 = 0.0;
const NO_OPTIONS: f32 = 0.0;

pub struct Hazard {
    position: Float3,
    radius: f32,
}

const MOVE_AWAY_COOLDOWN: i32 = 30;
const SEARCH_SCALE: f32 = 1.5;
const MOVE_DISTANCE: f32 = 120.0;
const CHECK_SCALE: f32 = 1.1;
const CIRCLE_DIVISIONS: i32 = 12;
const LINE_DIVISIONS: i32 = 6;
const MAX_DELTA_HEIGHT: f32 = 10.0;
const DEFAULT_ATTACK_RADIUS: f32 = 100.0;
const SAFE_SCORE: f64 = 1.0;
const UNSAFE_SCORE: f64 = -3.0;
const REJECT_SCORE: f64 = -1.0e11;

impl Robots {
    fn is_busy(&self, unit_id: i32, frame: i32) -> bool {
        self.moving_away
            .iter()
            .any(|(id, since)| *id == unit_id && since + MOVE_AWAY_COOLDOWN > frame)
    }

    fn is_tracked(&self, unit_id: i32) -> bool {
        self.moving_away.iter().any(|(id, _)| *id == unit_id)
    }
}

fn is_robot(unit_def_id: i32) -> bool {
    crate::defs::get().unit(unit_def_id).robot
}

fn is_inanimate(unit_def_id: i32) -> bool {
    crate::defs::get().unit(unit_def_id).effect
}

fn is_paralyzed(unit_id: i32) -> bool {
    spring::units_info::get_unit_health(unit_id)
        .map(|health| health.paralyze_damage >= health.max_health)
        .unwrap_or(false)
}

fn inanimate_radius(unit_id: i32, unit_def_id: i32) -> f32 {
    let defs = crate::defs::get();
    let key = if defs.fire == Some(unit_def_id) {
        crate::fire::SIZE_PARAM
    } else if defs.electrafi == Some(unit_def_id) {
        crate::electrafi::SIZE_PARAM
    } else {
        return DEFAULT_ATTACK_RADIUS;
    };
    spring::get_unit_rules_param_f32(unit_id, key).unwrap_or(DEFAULT_ATTACK_RADIUS)
}

fn hazards_near(position: Float3, radius: f32) -> Vec<Hazard> {
    units::units_in_sphere(position, radius)
        .into_iter()
        .filter_map(|unit_id| {
            let def_id = units::def_of(unit_id)?;
            if !is_inanimate(def_id) {
                return None;
            }
            Some(Hazard {
                position: units::position(unit_id)?,
                radius: inanimate_radius(unit_id, def_id),
            })
        })
        .collect()
}

/// `GetSafestLocation` in `unit_robots.lua`, quirks included: a better
/// direction clears the kept ones only once two are kept, so the first
/// direction tried can stay a candidate; and a direction off the map or up a
/// slope is only scored down, never ruled out.
fn safest_location(position: Float3, hazards: &[Hazard], velocity: Float3) -> Option<Float3> {
    if hazards.is_empty() {
        return None;
    }
    let mut theta = initial_theta(position, hazards);
    // (candidate, score); scores in f64 like Lua's, so the -1e10 of a ruled
    // down direction doesn't swallow the hazard scores.
    let mut good_points: Vec<(Float3, f64)> = Vec::new();
    for _ in 0..CIRCLE_DIVISIONS {
        let direction_x = theta.cos();
        let direction_z = theta.sin();
        let end_x = position.x + MOVE_DISTANCE * direction_x;
        let end_z = position.z + MOVE_DISTANCE * direction_z;
        let end_y = units::ground_height(end_x, end_z);
        let mut score = 0.0;
        if (position.y - end_y).abs() > MAX_DELTA_HEIGHT {
            score += REJECT_SCORE;
        }
        if !in_map(end_x, end_z) {
            score += REJECT_SCORE;
        }
        for step in 1..=LINE_DIVISIONS {
            let fraction = step as f32 / LINE_DIVISIONS as f32;
            let sample_x = position.x + MOVE_DISTANCE * CHECK_SCALE * direction_x * fraction;
            let sample_z = position.z + MOVE_DISTANCE * CHECK_SCALE * direction_z * fraction;
            score += location_score(sample_x, sample_z, hazards);
        }
        let candidate = Float3 {
            x: end_x,
            y: end_y,
            z: end_z,
        };
        let first_score = good_points.first().map(|(_, score)| *score);
        if first_score.is_none_or(|first| score >= first) {
            if good_points.len() > 1 && first_score.is_some_and(|first| score > first) {
                good_points.clear();
            }
            good_points.push((candidate, score));
        }
        theta += core::f32::consts::TAU / CIRCLE_DIVISIONS as f32;
    }
    let velocity_angle = velocity.x.atan2(velocity.z);
    let mut best = None;
    let mut best_angle = f32::MAX;
    for (candidate, _) in good_points {
        // Lua's unwrapped comparison: the kept point closest to the unit's
        // current travel angle.
        let angle = (candidate.x - position.x).atan2(candidate.z - position.z);
        let difference = (angle - velocity_angle).abs();
        if difference < best_angle {
            best_angle = difference;
            best = Some(candidate);
        }
    }
    best
}

fn initial_theta(position: Float3, hazards: &[Hazard]) -> f32 {
    let mut closest = f32::MAX;
    let mut theta = 0.0;
    for hazard in hazards {
        let dx = position.x - hazard.position.x;
        let dz = position.z - hazard.position.z;
        let distance = (dx * dx + dz * dz).sqrt();
        if distance > 0.0 && distance < closest {
            closest = distance;
            theta = (dx / distance).acos();
            if hazard.position.z > position.z {
                theta = -theta;
            }
        }
    }
    theta
}

fn location_score(x: f32, z: f32, hazards: &[Hazard]) -> f64 {
    hazards
        .iter()
        .map(|hazard| {
            let dx = x - hazard.position.x;
            let dz = z - hazard.position.z;
            if (dx * dx + dz * dz).sqrt() > hazard.radius {
                SAFE_SCORE
            } else {
                UNSAFE_SCORE
            }
        })
        .sum()
}

fn in_map(x: f32, z: f32) -> bool {
    let Ok(map) = spring::game::get_game_map_info(0) else {
        return false;
    };
    x >= 0.0 && x <= map.map_size_x as f32 && z >= 0.0 && z <= map.map_size_z as f32
}
