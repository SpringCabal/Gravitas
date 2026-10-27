use spring::Float3;

use super::Fire;
use super::effects;
use super::particles::Effect;

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
fn emit_effect(fire: &mut Fire, effect: &Effect, position: Float3, seed: u64) {
    let mut random = Random(seed as u32 ^ (seed >> 32) as u32);
    springcabal::particles::emit_effect(
        &mut fire.field,
        effect,
        [position.x, position.y, position.z],
        &mut || random.sample(),
    );
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
