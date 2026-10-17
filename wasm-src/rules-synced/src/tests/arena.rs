use alloc::rc::Rc;
use core::cell::Cell;

use spring::Float3;

use crate::units;

#[derive(Clone, Copy, Default)]
pub struct Arena {
    pub centre: Float3,
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

pub fn build(
    spring: &spring::Spring<'_, spring::RulesSynced>,
    generation: u32,
    built: impl FnOnce(&spring::Spring<'_, spring::RulesSynced>, Option<Arena>) + 'static,
) {
    let centre = flat_centre(spring);
    let arena = Rc::new(Cell::new(Arena {
        centre,
        generation,
        ..Arena::default()
    }));
    let layout: [(Field, &str, Rig, f32, f32); 29] = [
        (|a| &mut a.plate, PLATE, Rig::Plate, 0.0, 0.0),
        (|a| &mut a.gate, GATE, Rig::Plate, GATE_OFFSET, 0.0),
        (|a| &mut a.toggler, CRATE, Rig::Plate, 0.0, PARK_OFFSET),
        (|a| &mut a.pushed, CRATE, Rig::Impulse, 0.0, 0.0),
        (|a| &mut a.pusher, WALL, Rig::Impulse, -PUSH_SEPARATION, 0.0),
        (|a| &mut a.protected, WALL, Rig::Damage, 0.0, 0.0),
        (|a| &mut a.exposed, ROBOT, Rig::Damage, PARK_OFFSET, 0.0),
        (|a| &mut a.fire, FIRE, Rig::Fire, 0.0, 0.0),
        (|a| &mut a.burning, GRAVIT, Rig::Fire, 0.0, PARK_OFFSET),
        (|a| &mut a.electrafi, ELECTRAFI, Rig::Electrafi, 0.0, 0.0),
        (|a| &mut a.shocked, ROBOT, Rig::Electrafi, 0.0, PARK_OFFSET),
        (|a| &mut a.vaulter, GRAVIT, Rig::Jump, 0.0, 0.0),
        (|a| &mut a.dropped, ROBOT, Rig::Fall, -PARK_OFFSET, 0.0),
        (|a| &mut a.cushioned, ROBOT, Rig::Fall, PARK_OFFSET, 0.0),
        (
            |a| &mut a.thrown,
            ROBOT,
            Rig::Collision,
            THROW_DISTANCE,
            0.0,
        ),
        (|a| &mut a.anvil, CRATE, Rig::Collision, 0.0, 0.0),
        (|a| &mut a.barrel, BARREL, Rig::Barrel, 0.0, 0.0),
        (|a| &mut a.bystander, ROBOT, Rig::Barrel, BARREL_REACH, 0.0),
        (|a| &mut a.revivable, ROBOT, Rig::Respawn, 0.0, 0.0),
        (|a| &mut a.gunner, ROBOT, Rig::Shoot, 0.0, 0.0),
        (|a| &mut a.target, GRAVIT, Rig::Shoot, -TARGET_OFFSET, 0.0),
        (|a| &mut a.fleeing, ROBOT, Rig::Flee, 0.0, 0.0),
        (|a| &mut a.scare, FIRE, Rig::Flee, 0.0, SCARE_OFFSET),
        (|a| &mut a.dying_gravit, GRAVIT, Rig::Death, 0.0, 0.0),
        (|a| &mut a.dying_robot, ROBOT, Rig::Death, PARK_OFFSET, 0.0),
        (|a| &mut a.terrain_plate, PLATE, Rig::Terrain, 0.0, 0.0),
        (
            |a| &mut a.terrain_gate,
            GATE,
            Rig::Terrain,
            GATE_OFFSET,
            0.0,
        ),
        (
            |a| &mut a.terrain_toggler,
            CRATE,
            Rig::Terrain,
            0.0,
            PARK_OFFSET,
        ),
        (|a| &mut a.jumper, GRAVIT, Rig::Input, 0.0, 0.0),
    ];
    for (field, name, rig, offset_x, offset_z) in layout {
        spawn(spring, &arena, field, name, centre, rig, offset_x, offset_z);
    }
    crate::with_addon_context(|ctx| {
        ctx.commands().run(move |spring| {
            let arena = arena.get();
            if arena.missing() {
                built(spring, None);
                return;
            }
            size_field(
                spring,
                arena.fire,
                crate::gadgets::fire::behavior::SIZE_PARAM,
                FIRE_SIZE,
            );
            size_field(
                spring,
                arena.scare,
                crate::gadgets::fire::behavior::SIZE_PARAM,
                SCARE_SIZE,
            );
            size_field(
                spring,
                arena.electrafi,
                crate::gadgets::electrafi::behavior::SIZE_PARAM,
                ELECTRAFI_SIZE,
            );
            size_field(spring, arena.revivable, RESPAWNABLE_PARAM, 1.0);
            protect(spring, arena.protected);
            protect(spring, arena.pusher);
            built(spring, Some(arena));
        })
    });
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

    pub fn links(&self) -> [(i32, i32); 2] {
        [
            (self.plate, self.gate),
            (self.terrain_plate, self.terrain_gate),
        ]
    }
}

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

pub const RIG_SPACING: f32 = 450.0;
const RIG_COLUMNS: usize = 5;
const RIG_ROWS: usize = 3;
const PARK_OFFSET: f32 = 90.0;
const PUSH_SEPARATION: f32 = 120.0;
const TARGET_OFFSET: f32 = 200.0;
const THROW_DISTANCE: f32 = 70.0;
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
const ARENA_MARGIN: f32 = 400.0;

impl Arena {
    fn missing(&self) -> bool {
        self.units().iter().any(|unit_id| *unit_id < 0)
    }
}

type Field = fn(&mut Arena) -> &mut i32;

fn spawn(
    spring: &spring::Spring<'_, spring::RulesSynced>,
    arena: &Rc<Cell<Arena>>,
    field: Field,
    name: &str,
    centre: Float3,
    rig: Rig,
    offset_x: f32,
    offset_z: f32,
) {
    let position = rig_position(spring, centre, rig, offset_x, offset_z);
    let team = if name == GRAVIT {
        PLAYER_TEAM
    } else {
        ENEMY_TEAM
    };
    let arena = Rc::clone(arena);
    let store = move |unit_id: i32| {
        let mut built = arena.get();
        *field(&mut built) = unit_id;
        arena.set(built);
    };
    let Some(def_id) = crate::defs::get(spring).id(name) else {
        crate::with_addon_context(|ctx| ctx.commands().run(move |_spring| store(-1)));
        return;
    };
    units::create(spring, def_id, position, team).then(move |_spring, unit_id| {
        let Some(unit_id) = unit_id else {
            store(-1);
            return;
        };
        let _ = crate::with_addon_context(|ctx| {
            ctx.commands().give_order_to_unit(
                unit_id,
                spring::cmd::FIRE_STATE,
                &[FIRE_STATE_HOLD],
                0,
                0,
            )
        });
        let _ = crate::with_addon_context(|ctx| {
            ctx.commands().give_order_to_unit(
                unit_id,
                spring::cmd::MOVE_STATE,
                &[MOVE_STATE_HOLD],
                0,
                0,
            )
        });
        store(unit_id);
    });
}

fn rig_position(
    spring: &spring::Spring<'_, spring::RulesSynced>,
    centre: Float3,
    rig: Rig,
    offset_x: f32,
    offset_z: f32,
) -> Float3 {
    let index = rig as usize;
    let column = (index % RIG_COLUMNS) as f32 - (RIG_COLUMNS - 1) as f32 * 0.5;
    let row = (index / RIG_COLUMNS) as f32 - (RIG_ROWS - 1) as f32 * 0.5;
    let x = centre.x + column * RIG_SPACING + offset_x;
    let z = centre.z + row * RIG_SPACING + offset_z;
    Float3 {
        x,
        y: units::ground_height(spring, x, z),
        z,
    }
}

fn half_extents() -> (f32, f32) {
    (
        (RIG_COLUMNS - 1) as f32 * 0.5 * RIG_SPACING + ARENA_MARGIN,
        (RIG_ROWS - 1) as f32 * 0.5 * RIG_SPACING + ARENA_MARGIN,
    )
}

fn flat_centre(spring: &spring::Spring<'_, spring::RulesSynced>) -> Float3 {
    let Ok(map) = spring.game().get_game_map_info() else {
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
            let spread = spread_at(spring, x, z);
            if spread < best_spread {
                best_spread = spread;
                best = Float3 {
                    x,
                    y: units::ground_height(spring, x, z),
                    z,
                };
            }
            x += SAMPLE_STEP;
        }
        z += SAMPLE_STEP;
    }
    best
}

fn spread_at(spring: &spring::Spring<'_, spring::RulesSynced>, x: f32, z: f32) -> f32 {
    let mut lowest = f32::MAX;
    let mut highest = f32::MIN;
    let (half_x, half_z) = half_extents();
    let mut sample_z = z - half_z;
    while sample_z <= z + half_z {
        let mut sample_x = x - half_x;
        while sample_x <= x + half_x {
            let height = units::ground_height(spring, sample_x, sample_z);
            lowest = lowest.min(height);
            highest = highest.max(height);
            sample_x += SAMPLE_STEP;
        }
        sample_z += SAMPLE_STEP;
    }
    highest - lowest
}

fn size_field(
    spring: &spring::Spring<'_, spring::RulesSynced>,
    unit_id: i32,
    name: &str,
    value: f32,
) {
    let _ = spring.rules_params().set_unit_rules_param_f32(
        unit_id,
        name,
        value,
        units::RULES_PARAM_PRIVATE,
    );
}

fn protect(spring: &spring::Spring<'_, spring::RulesSynced>, unit_id: i32) {
    let _ = spring.rules_params().set_unit_rules_param_f32(
        unit_id,
        "invulnerable",
        1.0,
        units::RULES_PARAM_PRIVATE,
    );
}
