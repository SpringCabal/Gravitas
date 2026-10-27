use spring::Float3;

use super::Fire;
use super::effects;
use super::particles::{Effect, Particle};

pub fn burn(fire: &mut Fire, position: Float3, radius: f32, size_mult: f32, velocity: [f32; 2]) {
    let seed = seed_of(fire, position);
    raw_burn(fire, position, radius, size_mult, velocity, seed);
}

pub(super) fn raw_burn(
    fire: &mut Fire,
    position: Float3,
    radius: f32,
    size_mult: f32,
    velocity: [f32; 2],
    seed: u64,
) {
    let mut random = Random(seed as u32 ^ (seed >> 32) as u32);
    random.sample();
    random.sample();
    let mut flame = effects::UNITS_ON_FIRE;
    flame.size = radius * effects::ON_FIRE_SIZE_SCALE * size_mult;
    flame.life *= size_mult;
    flame.start_radius = radius * effects::ON_FIRE_DISC_SCALE;
    flame.force = [
        velocity[0] * effects::ON_FIRE_DRIFT,
        effects::ON_FIRE_LIFT + effects::ON_FIRE_LIFT_SPREAD * random.sample(),
        velocity[1] * effects::ON_FIRE_DRIFT,
    ];
    let mut smoke = effects::UNITS_ON_FIRE_SMOKE;
    smoke.size = flame.size * size_mult;
    smoke.life *= size_mult;
    smoke.start_radius = flame.start_radius;
    emit_effect(fire, &flame, position, seed.wrapping_add(1));
    emit_effect(fire, &smoke, position, seed.wrapping_add(2));
}

pub fn flame(fire: &mut Fire, position: Float3, radius: f32) {
    let seed = seed_of(fire, position);
    let speed = effects::FIRE_SPEED + 0.1 * signed(seed);
    let radius = radius * (1.0 + 0.1 * signed(next(seed)));
    raw_flame(fire, position, radius, speed, seed);
}

pub(super) fn raw_flame(fire: &mut Fire, position: Float3, radius: f32, speed: f32, seed: u64) {
    let mut heat = effects::TRAP_HEAT;
    heat.life = radius / effects::TRAP_FLAME_LIFE_DIVISOR;
    heat.emit_vector = [0.0, speed, 0.0];
    let mut flame = effects::TRAP_FLAME;
    flame.life = radius / effects::TRAP_FLAME_LIFE_DIVISOR;
    flame.emit_vector = [0.0, speed, 0.0];
    let mut sparks = effects::TRAP_SPARK;
    sparks.life = radius / effects::TRAP_SPARK_LIFE_DIVISOR;
    sparks.emit_vector = [0.0, speed, 0.0];
    emit_effect(fire, &heat, position, seed);
    emit_effect(fire, &flame, position, seed.wrapping_add(1));
    emit_effect(fire, &sparks, position, seed.wrapping_add(2));
}

pub fn electric_field(fire: &mut Fire, position: Float3, radius: f32) {
    let seed = seed_of(fire, position);
    let mut heat = effects::ELECTRIC_FIELD_HEAT;
    heat.life = radius;
    let mut effect = effects::ELECTRIC_FIELD;
    effect.life = radius;
    effect.rotation_speed = if unit_float(seed) > 0.5 {
        FAST_SPIN
    } else {
        SLOW_SPIN
    };
    emit_effect(fire, &heat, position, seed);
    emit_effect(fire, &effect, position, next(seed));
}

pub fn electric_spark(fire: &mut Fire, position: Float3, intensity: f32, velocity: [f32; 2]) {
    let seed = seed_of(fire, position);
    let life = (effects::SPARK_BASE_LIFE + effects::SPARK_LIFE_SPREAD * signed(seed))
        * intensity.max(0.05);

    let mut heat = effects::ELECTRIC_SPARK_HEAT;
    heat.force = [velocity[0] * 0.5, 0.6, velocity[1] * 0.5];
    heat.life = life;
    heat.size_growth += effects::SPARK_SIZE_GROWTH_SPREAD * signed(next(seed));
    emit_effect(fire, &heat, position, seed);

    let mut effect = effects::ELECTRIC_SPARK;
    effect.force = [velocity[0] * 0.65, 0.5, velocity[1] * 0.65];
    effect.life = life;
    effect.size_growth += effects::SPARK_SIZE_GROWTH_SPREAD * signed(next(seed));
    emit_effect(fire, &effect, position, next(seed));
}

const FAST_SPIN: f32 = 1.0;
const SLOW_SPIN: f32 = 0.2;
const DEGREES_TO_RADIANS: f32 = core::f32::consts::PI / 180.0;

fn emit_effect(fire: &mut Fire, effect: &Effect, position: Float3, seed: u64) {
    let force = [
        effect.force[0] * (effect.life + effect.life_spread),
        effect.force[1] * (effect.life + effect.life_spread),
        effect.force[2] * (effect.life + effect.life_spread),
    ];
    let mut random = Random(seed as u32 ^ (seed >> 32) as u32);
    fire.field.emit(
        (0..effect.count).map(|_| particle(effect, position, force, &mut random)),
        effect.life + effect.life_spread + effect.delay_spread,
    );
}

fn particle(effect: &Effect, position: Float3, force: [f32; 3], random: &mut Random) -> Particle {
    let azimuth = random.sample() * core::f32::consts::TAU;
    let cone = random.sample() * effect.emit_rotation_spread * DEGREES_TO_RADIANS;
    let life = effect.life + random.sample() * effect.life_spread;
    let travel = (effect.speed + random.sample() * effect.speed_spread) * life;
    let delay = random.sample() * effect.delay_spread;
    let size = effect.size + random.sample() * effect.size_spread;
    let (rotation, rotation_end) = if effect.distortion {
        (0.0, 0.0)
    } else {
        (
            random.sample() * effect.rotation_spread,
            (effect.rotation_speed + random.sample() * effect.rotation_speed_spread) * life,
        )
    };
    let (disc_angle, disc) = if effect.start_radius > 0.0 {
        (
            random.sample() * core::f32::consts::TAU,
            random.sample() * effect.start_radius,
        )
    } else {
        (0.0, 0.0)
    };
    let direction = emit_direction(effect.emit_vector, cone, azimuth, travel);
    Particle {
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
        birth: 0.0,
        death: 0.0,
        effect: 0,
        delay,
        life: life.max(MINIMUM_LIFE),
        size,
        size_end: size + effect.size_growth * life,
        rotation,
        rotation_end,
        speed_exp: effect.speed_exp,
        size_exp: effect.size_exp,
        rotation_exp: effect.rotation_exp,
        animation_x: if effect.distortion {
            (random.sample() * 2.0 - 1.0).floor()
        } else {
            0.0
        },
        animation_y: if effect.distortion {
            (random.sample() * 2.0 - 1.0).floor()
        } else {
            0.0
        },
        distortion: effect.distortion,
        distortion_strength: effect.distortion_strength,
        distortion_scale: effect.distortion_scale,
        distortion_heat: effect.distortion_heat,
        colormap: effect.colormap,
        palette: 0,
        serial: 0,
        texture: effect.texture,
    }
}

struct Random(u32);

impl Random {
    fn sample(&mut self) -> f32 {
        #[cfg(feature = "fire-probe")]
        {
            self.0 = (self.0 * 251 + 1) % 65521;
            self.0 as f32 / 65521.0
        }
        #[cfg(not(feature = "fire-probe"))]
        {
            self.0 = self.0.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
            (self.0 >> 8) as f32 / 16_777_216.0
        }
    }
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

fn emit_direction(vector: [f32; 3], cone: f32, azimuth: f32, distance: f32) -> [f32; 3] {
    let up = vector;
    let right = [up[1], up[2], -up[0]];
    let right = cross(up, right);
    let forward = cross(up, right);
    let (sin_cone, cos_cone) = cone.sin_cos();
    let (sin_azimuth, cos_azimuth) = azimuth.sin_cos();
    let a = cos_cone;
    let b = cos_azimuth * sin_cone;
    let c = sin_azimuth * sin_cone;
    let local = [
        (up[0] * a - right[0] * b + forward[0] * c) * distance,
        (up[1] * a - right[1] * b + forward[1] * c) * distance,
        (up[2] * a - right[2] * b + forward[2] * c) * distance,
    ];
    emit(vector, local)
}

fn cross(a: [f32; 3], b: [f32; 3]) -> [f32; 3] {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}

fn seed_of(fire: &mut Fire, position: Float3) -> u64 {
    fire.emissions = fire.emissions.wrapping_add(1);
    let serial = fire.emissions;
    position.x.to_bits() as u64
        ^ ((position.z.to_bits() as u64) << 16)
        ^ serial.wrapping_mul(2_862_933_555_777_941_757)
}

fn next(seed: u64) -> u64 {
    seed.wrapping_mul(6_364_136_223_846_793_005)
        .wrapping_add(1_442_695_040_888_963_407)
}

fn unit_float(seed: u64) -> f32 {
    let mut mixed = seed;
    mixed = (mixed ^ (mixed >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    mixed = (mixed ^ (mixed >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
    mixed ^= mixed >> 31;
    ((mixed >> 40) as u32) as f32 / 16_777_216.0
}

fn signed(seed: u64) -> f32 {
    2.0 * unit_float(seed) - 1.0
}
