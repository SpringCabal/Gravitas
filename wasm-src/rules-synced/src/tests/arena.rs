use spring::types::Float3;

use crate::units;

/// The e2e cases' units. Each case has its own rig, an area of the arena no
/// other case's units reach, so the cases can run at the same time.
#[derive(Clone, Copy)]
pub struct Arena {
    pub centre: Float3,
    /// Bumped every time the arena is built (a restart builds it again).
    pub generation: u32,
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
    pub thrown: i32,
    pub anvil: i32,
    pub barrel: i32,
    pub bystander: i32,
    pub revivable: i32,
    pub fleeing: i32,
    pub scare: i32,
    pub gunner: i32,
    pub target: i32,
    pub dying_gravit: i32,
    pub dying_robot: i32,
    pub terrain_plate: i32,
    pub terrain_gate: i32,
    pub terrain_toggler: i32,
}

pub fn build(generation: u32) -> Option<Arena> {
    let centre = flat_centre();
    let arena = Arena {
        centre,
        generation,
        plate: spawn(PLATE, centre, Rig::Plate, 0.0, 0.0),
        gate: spawn(GATE, centre, Rig::Plate, GATE_OFFSET, 0.0),
        toggler: spawn(CRATE, centre, Rig::Plate, 0.0, PARK_OFFSET),
        pushed: spawn(CRATE, centre, Rig::Impulse, 0.0, 0.0),
        pusher: spawn(WALL, centre, Rig::Impulse, -PUSH_SEPARATION, 0.0),
        protected: spawn(WALL, centre, Rig::Damage, 0.0, 0.0),
        exposed: spawn(ROBOT, centre, Rig::Damage, PARK_OFFSET, 0.0),
        fire: spawn(FIRE, centre, Rig::Fire, 0.0, 0.0),
        burning: spawn(GRAVIT, centre, Rig::Fire, 0.0, PARK_OFFSET),
        electrafi: spawn(ELECTRAFI, centre, Rig::Electrafi, 0.0, 0.0),
        shocked: spawn(ROBOT, centre, Rig::Electrafi, 0.0, PARK_OFFSET),
        vaulter: spawn(GRAVIT, centre, Rig::Jump, 0.0, 0.0),
        dropped: spawn(ROBOT, centre, Rig::Fall, -PARK_OFFSET, 0.0),
        cushioned: spawn(ROBOT, centre, Rig::Fall, PARK_OFFSET, 0.0),
        thrown: spawn(ROBOT, centre, Rig::Collision, THROW_DISTANCE, 0.0),
        anvil: spawn(CRATE, centre, Rig::Collision, 0.0, 0.0),
        barrel: spawn(BARREL, centre, Rig::Barrel, 0.0, 0.0),
        bystander: spawn(ROBOT, centre, Rig::Barrel, BARREL_REACH, 0.0),
        revivable: spawn(ROBOT, centre, Rig::Respawn, 0.0, 0.0),
        gunner: spawn(ROBOT, centre, Rig::Shoot, 0.0, 0.0),
        target: spawn(GRAVIT, centre, Rig::Shoot, -TARGET_OFFSET, 0.0),
        fleeing: spawn(ROBOT, centre, Rig::Flee, 0.0, 0.0),
        scare: spawn(FIRE, centre, Rig::Flee, 0.0, SCARE_OFFSET),
        dying_gravit: spawn(GRAVIT, centre, Rig::Death, 0.0, 0.0),
        dying_robot: spawn(ROBOT, centre, Rig::Death, PARK_OFFSET, 0.0),
        terrain_plate: spawn(PLATE, centre, Rig::Terrain, 0.0, 0.0),
        terrain_gate: spawn(GATE, centre, Rig::Terrain, GATE_OFFSET, 0.0),
        terrain_toggler: spawn(CRATE, centre, Rig::Terrain, 0.0, PARK_OFFSET),
        // Last of the Gravits: the player gadget adopts it.
        jumper: spawn(GRAVIT, centre, Rig::Input, 0.0, 0.0),
    };
    if arena.missing() {
        return None;
    }
    size_field(arena.fire, crate::fire::SIZE_PARAM, FIRE_SIZE);
    size_field(arena.scare, crate::fire::SIZE_PARAM, SCARE_SIZE);
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
    pub fn units(&self) -> [i32; 29] {
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
            self.thrown,
            self.anvil,
            self.barrel,
            self.bystander,
            self.revivable,
            self.fleeing,
            self.scare,
            self.gunner,
            self.target,
            self.dying_gravit,
            self.dying_robot,
            self.terrain_plate,
            self.terrain_gate,
            self.terrain_toggler,
        ]
    }

    /// Link each plate to its gate (on the frame after the build).
    pub fn links(&self) -> [(i32, i32); 2] {
        [
            (self.plate, self.gate),
            (self.terrain_plate, self.terrain_gate),
        ]
    }
}

/// The rigs, laid out on a grid. The shoot and flee rigs are on the top row:
/// their robots walk up, out of the grid.
#[derive(Clone, Copy)]
enum Rig {
    Flee,
    Shoot,
    Jump,
    Input,
    Electrafi,
    Plate,
    Terrain,
    Impulse,
    Damage,
    Death,
    Fire,
    Fall,
    Collision,
    Barrel,
    Respawn,
}

/// Rig centres are this far apart, so a robot that reacts to a hazard and
/// walks off stays in its own rig.
pub const RIG_SPACING: f32 = 450.0;
const RIG_COLUMNS: usize = 5;
const RIG_ROWS: usize = 3;
const PARK_OFFSET: f32 = 90.0;
const PUSH_SEPARATION: f32 = 120.0;
const TARGET_OFFSET: f32 = 200.0;
const THROW_DISTANCE: f32 = 70.0;
/// Behind the fleeing robot (it walks up), and small enough not to reach it.
const SCARE_OFFSET: f32 = 150.0;
const SCARE_SIZE: f32 = 50.0;

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
/// How far past the outer rig centres the flat ground has to reach: the
/// top row's robots walk and jump up to 300 elmos up.
const ARENA_MARGIN: f32 = 400.0;

impl Arena {
    fn missing(&self) -> bool {
        self.units().iter().any(|unit_id| *unit_id < 0)
    }
}

fn spawn(name: &str, centre: Float3, rig: Rig, offset_x: f32, offset_z: f32) -> i32 {
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

fn rig_position(centre: Float3, rig: Rig, offset_x: f32, offset_z: f32) -> Float3 {
    let index = rig as usize;
    let column = (index % RIG_COLUMNS) as f32 - (RIG_COLUMNS - 1) as f32 * 0.5;
    let row = (index / RIG_COLUMNS) as f32 - (RIG_ROWS - 1) as f32 * 0.5;
    let x = centre.x + column * RIG_SPACING + offset_x;
    let z = centre.z + row * RIG_SPACING + offset_z;
    Float3 {
        x,
        y: units::ground_height(x, z),
        z,
    }
}

/// Half the arena's width and depth: the rig grid, and a margin around it.
fn half_extents() -> (f32, f32) {
    (
        (RIG_COLUMNS - 1) as f32 * 0.5 * RIG_SPACING + ARENA_MARGIN,
        (RIG_ROWS - 1) as f32 * 0.5 * RIG_SPACING + ARENA_MARGIN,
    )
}

/// The flattest spot on the map for the whole arena.
fn flat_centre() -> Float3 {
    let Ok(map) = spring::game::get_game_map_info(0) else {
        return Float3::default();
    };
    let width = map.map_size_x as f32;
    let depth = map.map_size_z as f32;
    let (half_x, half_z) = half_extents();
    let mut best = Float3 {
        x: width * 0.5,
        y: 0.0,
        z: depth * 0.5,
    };
    let mut best_spread = f32::MAX;
    let mut z = half_z;
    while z <= depth - half_z {
        let mut x = half_x;
        while x <= width - half_x {
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
    let (half_x, half_z) = half_extents();
    let mut sample_z = z - half_z;
    while sample_z <= z + half_z {
        let mut sample_x = x - half_x;
        while sample_x <= x + half_x {
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
