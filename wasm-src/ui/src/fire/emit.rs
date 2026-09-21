use spring::types::Float3;

use crate::state;

use super::effects;
use super::particles::{Effect, Particle};

pub fn update(frames: f32) {
    let _ = state::with_state(|ui| ui.fire.advance(frames));
}

pub fn burn(position: Float3, radius: f32, size_mult: f32, velocity: [f32; 2]) {
    let seed = seed_of(position);
    let mut flame = effects::UNITS_ON_FIRE;
    flame.size = radius * effects::ON_FIRE_SIZE_SCALE * size_mult;
    flame.life *= size_mult;
    flame.start_radius = radius * effects::ON_FIRE_DISC_SCALE;
    flame.force = [
        velocity[0] * effects::ON_FIRE_DRIFT,
        effects::ON_FIRE_LIFT + effects::ON_FIRE_LIFT_SPREAD * unit_float(seed),
        velocity[1] * effects::ON_FIRE_DRIFT,
    ];
    let mut smoke = effects::UNITS_ON_FIRE_SMOKE;
    smoke.size = flame.size * size_mult;
    smoke.life *= size_mult;
    smoke.start_radius = flame.start_radius;
    smoke.force = flame.force;
    emit_effect(&flame, position, seed);
    emit_effect(&smoke, position, next(seed));
}

pub fn flame(position: Float3, radius: f32) {
    let mut heat = effects::TRAP_HEAT;
    heat.life = radius / effects::TRAP_FLAME_LIFE_DIVISOR;
    let mut fire = effects::TRAP_FLAME;
    fire.life = radius / effects::TRAP_FLAME_LIFE_DIVISOR;
    let mut sparks = effects::TRAP_SPARK;
    sparks.life = radius / effects::TRAP_SPARK_LIFE_DIVISOR;
    let seed = seed_of(position);
    emit_effect(&heat, position, seed);
    emit_effect(&fire, position, next(seed));
    emit_effect(&sparks, position, next(next(seed)));
}

pub fn electric_field(position: Float3, radius: f32) {
    let seed = seed_of(position);
    let mut heat = effects::ELECTRIC_FIELD_HEAT;
    heat.life = radius;
    let mut effect = effects::ELECTRIC_FIELD;
    effect.life = radius;
    effect.rotation_speed = if unit_float(seed) > 0.5 {
        FAST_SPIN
    } else {
        SLOW_SPIN
    };
    emit_effect(&heat, position, seed);
    emit_effect(&effect, position, next(seed));
}

pub fn electric_spark(position: Float3, intensity: f32, velocity: [f32; 2]) {
    let seed = seed_of(position);
    let life = (effects::SPARK_BASE_LIFE + effects::SPARK_LIFE_SPREAD * signed(seed))
        * intensity.max(0.05);

    let mut heat = effects::ELECTRIC_SPARK_HEAT;
    heat.force = [velocity[0] * 0.5, 0.6, velocity[1] * 0.5];
    heat.life = life;
    heat.size_growth += effects::SPARK_SIZE_GROWTH_SPREAD * signed(next(seed));
    emit_effect(&heat, position, seed);

    let mut effect = effects::ELECTRIC_SPARK;
    effect.force = [velocity[0] * 0.65, 0.5, velocity[1] * 0.65];
    effect.life = life;
    effect.size_growth += effects::SPARK_SIZE_GROWTH_SPREAD * signed(next(seed));
    effect.start_radius = effects::SPARK_DISC;
    emit_effect(&effect, position, next(seed));
}

const FAST_SPIN: f32 = 1.0;
const SLOW_SPIN: f32 = 0.2;
const DEGREES_TO_RADIANS: f32 = core::f32::consts::PI / 180.0;

fn emit_effect(effect: &Effect, position: Float3, seed: u64) {
    let force = [
        effect.force[0] * (effect.life + effect.life_spread),
        effect.force[1] * (effect.life + effect.life_spread),
        effect.force[2] * (effect.life + effect.life_spread),
    ];
    let mut seed = seed;
    for _ in 0..effect.count {
        seed = next(seed);
        spawn(effect, position, force, seed);
    }
}

fn spawn(effect: &Effect, position: Float3, force: [f32; 3], seed: u64) {
    let azimuth = unit_float(seed) * core::f32::consts::TAU;
    let cone = unit_float(next(seed)) * effect.emit_rotation_spread * DEGREES_TO_RADIANS;
    let life = effect.life + unit_float(next(next(seed))) * effect.life_spread;
    let travel = (effect.speed + unit_float(seed.wrapping_add(7)) * effect.speed_spread) * life;
    let size = effect.size + unit_float(seed.wrapping_add(11)) * effect.size_spread;
    let disc = unit_float(seed.wrapping_add(13)) * effect.start_radius;
    let disc_angle = unit_float(seed.wrapping_add(17)) * core::f32::consts::TAU;
    let local = [
        cone.sin() * azimuth.cos() * travel,
        cone.cos() * travel,
        cone.sin() * azimuth.sin() * travel,
    ];
    let direction = emit(effect.emit_vector, local);
    let particle = Particle {
        x: position.x + disc * disc_angle.sin(),
        y: position.y,
        z: position.z + disc * disc_angle.cos(),
        direction_x: direction[0],
        direction_y: direction[1],
        direction_z: direction[2],
        force_x: force[0],
        force_y: force[1],
        force_z: force[2],
        age: 0.0,
        delay: unit_float(seed.wrapping_add(19)) * effect.delay_spread,
        life: life.max(MINIMUM_LIFE),
        size,
        size_end: size + effect.size_growth * life,
        rotation: unit_float(seed.wrapping_add(23)) * effect.rotation_spread,
        rotation_end: (effect.rotation_speed
            + unit_float(seed.wrapping_add(29)) * effect.rotation_speed_spread)
            * life,
        speed_exp: effect.speed_exp,
        size_exp: effect.size_exp,
        rotation_exp: effect.rotation_exp,
        distortion: effect.distortion,
        distortion_strength: effect.distortion_strength,
        distortion_scale: effect.distortion_scale,
        distortion_heat: effect.distortion_heat,
        colormap: effect.colormap,
        texture: effect.texture,
    };
    let _ = state::with_state(|ui| ui.fire.spawn(particle));
}

const MINIMUM_LIFE: f32 = 1.0;

fn emit(vector: [f32; 3], local: [f32; 3]) -> [f32; 3] {
    let [x, y, z] = vector;
    let matrix = [
        x * x,
        x * y - z,
        x * z + y,
        x * y + z,
        y * y,
        y * z - x,
        x * z - y,
        y * z + x,
        z * z,
    ];
    [
        matrix[0] * local[0] + matrix[1] * local[1] + matrix[2] * local[2],
        matrix[3] * local[0] + matrix[4] * local[1] + matrix[5] * local[2],
        matrix[6] * local[0] + matrix[7] * local[1] + matrix[8] * local[2],
    ]
}

fn seed_of(position: Float3) -> u64 {
    let serial = state::with_state(|ui| {
        ui.fire_seed = ui.fire_seed.wrapping_add(1);
        ui.fire_seed
    })
    .unwrap_or(0);
    position.x.to_bits() as u64
        ^ ((position.z.to_bits() as u64) << 16)
        ^ serial.wrapping_mul(2_862_933_555_777_941_757)
}

fn next(seed: u64) -> u64 {
    seed.wrapping_mul(6_364_136_223_846_793_005)
        .wrapping_add(1_442_695_040_888_963_407)
}

fn unit_float(seed: u64) -> f32 {
    ((next(seed) >> 40) as u32) as f32 / 16_777_216.0
}

fn signed(seed: u64) -> f32 {
    2.0 * unit_float(seed) - 1.0
}
