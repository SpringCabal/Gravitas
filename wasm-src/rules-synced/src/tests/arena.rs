use spring::types::Float3;

use crate::units;

#[derive(Clone, Copy)]
pub struct Arena {
    pub centre: Float3,
    pub plate: i32,
    pub gate: i32,
    pub toggler: i32,
    pub pushed: i32,
    pub pusher: i32,
    pub protected: i32,
    pub exposed: i32,
    pub fire: i32,
    pub burning: i32,
    pub electrafi: i32,
    pub shocked: i32,
    pub jumper: i32,
    pub vaulter: i32,
    pub dropped: i32,
    pub cushioned: i32,
    pub barrel: i32,
    pub bystander: i32,
    pub revivable: i32,
}

pub fn build() -> Option<Arena> {
    let centre = flat_centre();
    let arena = Arena {
        centre,
        plate: spawn(PLATE, centre, RIG_PLATE, 0.0, 0.0),
        gate: spawn(GATE, centre, RIG_PLATE, GATE_OFFSET, 0.0),
        toggler: spawn(CRATE, centre, RIG_PLATE, 0.0, PARK_OFFSET),
        pushed: spawn(CRATE, centre, RIG_IMPULSE, 0.0, 0.0),
        pusher: spawn(WALL, centre, RIG_IMPULSE, -PUSH_SEPARATION, 0.0),
        protected: spawn(WALL, centre, RIG_DAMAGE, 0.0, 0.0),
        exposed: spawn(ROBOT, centre, RIG_DAMAGE, PARK_OFFSET, 0.0),
        fire: spawn(FIRE, centre, RIG_FIRE, 0.0, 0.0),
        burning: spawn(GRAVIT, centre, RIG_FIRE, 0.0, PARK_OFFSET),
        electrafi: spawn(ELECTRAFI, centre, RIG_ELECTRAFI, 0.0, 0.0),
        shocked: spawn(ROBOT, centre, RIG_ELECTRAFI, 0.0, PARK_OFFSET),
        vaulter: spawn(GRAVIT, centre, RIG_JUMP, 0.0, 0.0),
        dropped: spawn(ROBOT, centre, RIG_FALL, -PARK_OFFSET, 0.0),
        cushioned: spawn(ROBOT, centre, RIG_FALL, PARK_OFFSET, 0.0),
        barrel: spawn(BARREL, centre, RIG_BARREL, 0.0, 0.0),
        bystander: spawn(ROBOT, centre, RIG_BARREL, BARREL_REACH, 0.0),
        revivable: spawn(ROBOT, centre, RIG_RESPAWN, 0.0, 0.0),
        jumper: spawn(GRAVIT, centre, RIG_INPUT, 0.0, 0.0),
    };
    if arena.missing() {
        return None;
    }
    size_field(arena.fire, crate::fire::SIZE_PARAM, FIRE_SIZE);
    size_field(
        arena.electrafi,
        crate::electrafi::SIZE_PARAM,
        ELECTRAFI_SIZE,
    );
    size_field(arena.revivable, RESPAWNABLE_PARAM, 1.0);
    protect(arena.protected);
    protect(arena.pusher);
    Some(arena)
}

impl Arena {
    pub fn units(&self) -> [i32; 18] {
        [
            self.plate,
            self.gate,
            self.toggler,
            self.pushed,
            self.pusher,
            self.protected,
            self.exposed,
            self.fire,
            self.burning,
            self.electrafi,
            self.shocked,
            self.jumper,
            self.vaulter,
            self.dropped,
            self.cushioned,
            self.barrel,
            self.bystander,
            self.revivable,
        ]
    }
}

const RIG_PLATE: usize = 0;
const RIG_FIRE: usize = 1;
const RIG_ELECTRAFI: usize = 2;
const RIG_FALL: usize = 3;
const RIG_IMPULSE: usize = 4;
const RIG_DAMAGE: usize = 5;
const RIG_BARREL: usize = 6;
const RIG_RESPAWN: usize = 7;
const RIG_JUMP: usize = 8;
const RIG_INPUT: usize = 9;

// Keep independent cases apart even when a robot reacts to a hazard and
// travels before another case's area-of-effect check runs.
const RIG_SPACING: f32 = 400.0;
const RIG_COLUMNS: usize = 4;
const PARK_OFFSET: f32 = 90.0;
const PUSH_SEPARATION: f32 = 120.0;

const PLATE: &str = "plate";
const GATE: &str = "gate";
const CRATE: &str = "crate";
const WALL: &str = "wall1";
const ROBOT: &str = "projector";
const BARREL: &str = "barrel";
const FIRE: &str = "fire";
const ELECTRAFI: &str = "electrafi";
const GRAVIT: &str = "gravit";

const GATE_OFFSET: f32 = 110.0;
const FIRE_STATE_HOLD: f32 = 0.0;
const MOVE_STATE_HOLD: f32 = 0.0;
const RESPAWNABLE_PARAM: &str = "respawnable";
const BARREL_REACH: f32 = 60.0;
const FIRE_SIZE: f32 = 150.0;
const ELECTRAFI_SIZE: f32 = 100.0;
const PLAYER_TEAM: i32 = 0;
const ENEMY_TEAM: i32 = 1;
const SAMPLE_STEP: f32 = 128.0;
const ARENA_SPAN: f32 = 1100.0;
const MAP_MARGIN: f32 = 600.0;

impl Arena {
    fn missing(&self) -> bool {
        self.units().iter().any(|unit_id| *unit_id < 0)
    }
}

fn spawn(name: &str, centre: Float3, rig: usize, offset_x: f32, offset_z: f32) -> i32 {
    let position = rig_position(centre, rig, offset_x, offset_z);
    let team = if name == GRAVIT {
        PLAYER_TEAM
    } else {
        ENEMY_TEAM
    };
    let Some(unit_id) = crate::defs::get()
        .id(name)
        .and_then(|def_id| units::create(def_id, position, team))
    else {
        return -1;
    };
    let _ = spring::give_order_to_unit(unit_id, spring::cmd::FIRE_STATE, &[FIRE_STATE_HOLD], 0, 0);
    let _ = spring::give_order_to_unit(unit_id, spring::cmd::MOVE_STATE, &[MOVE_STATE_HOLD], 0, 0);
    unit_id
}

fn rig_position(centre: Float3, rig: usize, offset_x: f32, offset_z: f32) -> Float3 {
    let column = (rig % RIG_COLUMNS) as f32 - 1.5;
    let row = (rig / RIG_COLUMNS) as f32 - 1.0;
    let x = centre.x + column * RIG_SPACING + offset_x;
    let z = centre.z + row * RIG_SPACING + offset_z;
    Float3 {
        x,
        y: units::ground_height(x, z),
        z,
    }
}

fn flat_centre() -> Float3 {
    let Ok(map) = spring::game::get_game_map_info(0) else {
        return Float3::default();
    };
    let width = map.map_size_x as f32;
    let depth = map.map_size_z as f32;
    let mut best = Float3 {
        x: width * 0.5,
        y: 0.0,
        z: depth * 0.5,
    };
    let mut best_spread = f32::MAX;
    let mut z = MAP_MARGIN;
    while z < depth - MAP_MARGIN {
        let mut x = MAP_MARGIN;
        while x < width - MAP_MARGIN {
            let spread = spread_at(x, z);
            if spread < best_spread {
                best_spread = spread;
                best = Float3 {
                    x,
                    y: units::ground_height(x, z),
                    z,
                };
            }
            x += SAMPLE_STEP;
        }
        z += SAMPLE_STEP;
    }
    best
}

fn spread_at(x: f32, z: f32) -> f32 {
    let mut lowest = f32::MAX;
    let mut highest = f32::MIN;
    let half = ARENA_SPAN * 0.5;
    let mut sample_z = z - half;
    while sample_z <= z + half {
        let mut sample_x = x - half;
        while sample_x <= x + half {
            let height = units::ground_height(sample_x, sample_z);
            lowest = lowest.min(height);
            highest = highest.max(height);
            sample_x += SAMPLE_STEP;
        }
        sample_z += SAMPLE_STEP;
    }
    highest - lowest
}

fn size_field(unit_id: i32, name: &str, value: f32) {
    let _ = spring::set_unit_rules_param_f32(unit_id, name, value, units::RULES_PARAM_PRIVATE);
}

fn protect(unit_id: i32) {
    let _ =
        spring::set_unit_rules_param_f32(unit_id, "invulnerable", 1.0, units::RULES_PARAM_PRIVATE);
}
