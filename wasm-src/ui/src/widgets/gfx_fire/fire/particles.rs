use alloc::collections::VecDeque;
use alloc::vec::Vec;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Texture {
    Flame,
    Electric,
    Smoke,
}

impl Texture {
    const fn index(self) -> usize {
        self as usize
    }
    pub fn path(self) -> &'static str {
        match self {
            Self::Flame => "bitmaps/GPL/flame.png",
            Self::Electric => "bitmaps/GPL/electricity.png",
            Self::Smoke => "bitmaps/GPL/smoke_orange.png",
        }
    }
}

#[derive(Clone, Copy)]
pub struct Effect {
    pub count: i32,
    pub life: f32,
    pub life_spread: f32,
    pub delay_spread: f32,
    pub force: [f32; 3],
    pub speed: f32,
    pub speed_spread: f32,
    pub speed_exp: f32,
    pub size: f32,
    pub size_spread: f32,
    pub size_growth: f32,
    pub size_exp: f32,
    pub rotation_speed: f32,
    pub rotation_speed_spread: f32,
    pub rotation_spread: f32,
    pub rotation_exp: f32,
    pub distortion: bool,
    pub distortion_strength: f32,
    pub distortion_scale: f32,
    pub distortion_heat: f32,
    pub emit_rotation_spread: f32,
    pub emit_vector: [f32; 3],
    pub start_radius: f32,
    pub colormap: &'static [[f32; 4]],
    pub texture: Texture,
}

#[derive(Clone, Copy)]
pub struct Particle {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub direction_x: f32,
    pub direction_y: f32,
    pub direction_z: f32,
    pub force_x: f32,
    pub force_y: f32,
    pub force_z: f32,
    pub age: f32,
    pub birth: f32,
    pub death: f32,
    pub effect: u64,
    pub delay: f32,
    pub life: f32,
    pub size: f32,
    pub size_end: f32,
    pub rotation: f32,
    pub rotation_end: f32,
    pub speed_exp: f32,
    pub size_exp: f32,
    pub rotation_exp: f32,
    pub animation_x: f32,
    pub animation_y: f32,
    pub distortion: bool,
    pub distortion_strength: f32,
    pub distortion_scale: f32,
    pub distortion_heat: f32,
    pub colormap: &'static [[f32; 4]],
    pub palette: usize,
    pub serial: u64,
    pub texture: Texture,
}

pub struct Sample {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub size: f32,
    pub rotation: f32,
    pub animation_x: f32,
    pub animation_y: f32,
    pub color: [f32; 4],
    pub distortion_strength: f32,
    pub distortion_scale: f32,
    pub distortion_heat: f32,
}

pub(super) struct TimedParticle<'a> {
    particle: &'a Particle,
    pub age: f32,
}
impl core::ops::Deref for TimedParticle<'_> {
    type Target = Particle;
    fn deref(&self) -> &Particle {
        self.particle
    }
}

struct Lifetime {
    count: usize,
    death: f32,
    serial: u64,
    texture: usize,
}

pub struct Field {
    particles: [VecDeque<Particle>; 4],
    lifetimes: [VecDeque<Lifetime>; 4],
    first_texture: [u64; 3],
    palettes: Vec<&'static [[f32; 4]]>,
    pub(super) colors: Vec<f32>,
    pub(super) epoch: u32,
    pub(super) serial: u64,
    pub(super) elapsed: f32,
}

impl Default for Field {
    fn default() -> Self {
        use core::sync::atomic::{AtomicU32, Ordering};
        static NEXT: AtomicU32 = AtomicU32::new(1);
        Self {
            particles: Default::default(),
            lifetimes: Default::default(),
            first_texture: [u64::MAX; 3],
            palettes: Vec::new(),
            colors: Vec::new(),
            epoch: NEXT.fetch_add(1, Ordering::Relaxed),
            serial: 0,
            elapsed: 0.0,
        }
    }
}

impl Field {
    pub fn emit(&mut self, particles: impl Iterator<Item = Particle>, lifetime: f32) {
        let serial = self.serial + 1;
        let death = self.elapsed + lifetime;
        let mut counts = [0; 4];
        let mut textures = [0; 4];
        for mut p in particles {
            p.birth = self.elapsed;
            p.death = death;
            p.effect = serial;
            let group = if p.distortion { 3 } else { p.texture.index() };
            counts[group] += 1;
            textures[group] = p.texture.index();
            self.spawn(p);
        }
        for (i, count) in counts.into_iter().enumerate() {
            if count > 0 {
                self.lifetimes[i].push_back(Lifetime {
                    count,
                    death,
                    serial,
                    texture: textures[i],
                });
            }
        }
    }

    fn spawn(&mut self, mut particle: Particle) {
        self.serial += 1;
        particle.serial = self.serial;
        let texture = particle.texture.index();
        self.first_texture[texture] = self.first_texture[texture].min(particle.serial);
        let group = if particle.distortion { 3 } else { texture };
        let mut offset = 0;
        for colors in &self.palettes {
            if core::ptr::eq(*colors, particle.colormap) {
                particle.palette = offset;
                self.particles[group].push_back(particle);
                return;
            }
            offset += colors.len();
        }
        self.palettes.push(particle.colormap);
        self.colors
            .extend_from_slice(particle.colormap.as_flattened());
        particle.palette = offset;
        self.particles[group].push_back(particle);
    }

    pub fn advance(&mut self, frames: f32) {
        self.elapsed += frames;
        let mut first = [u64::MAX; 3];
        for (group, lifetimes) in self.particles.iter_mut().zip(&mut self.lifetimes) {
            // Match LUPS effect lifetimes. The GPU clips individual particles;
            // CPU updates touch one emission record, rather than each particle.
            while lifetimes.front().is_some_and(|p| p.death < self.elapsed) {
                let span = lifetimes.pop_front().unwrap();
                group.drain(..span.count);
            }
            for span in lifetimes.iter().filter(|p| p.death >= self.elapsed) {
                first[span.texture] = first[span.texture].min(span.serial);
            }
        }
        self.first_texture = first;
    }

    pub(super) fn drawable(&self, texture: Texture) -> impl Iterator<Item = TimedParticle<'_>> {
        self.timed_particles(Some(texture))
    }
    fn timed_particles(&self, texture: Option<Texture>) -> impl Iterator<Item = TimedParticle<'_>> {
        self.gpu_particles(texture).iter().filter_map(|p| {
            let age = self.elapsed - p.birth;
            (age > p.delay && age <= p.delay + p.life).then_some(TimedParticle { particle: p, age })
        })
    }
    pub fn visible(&self, texture: Texture) -> impl Iterator<Item = Sample> {
        self.drawable(texture).map(|p| sample(p.particle, p.age))
    }

    pub(super) fn gpu_particles(&self, texture: Option<Texture>) -> &VecDeque<Particle> {
        &self.particles[texture.map_or(3, Texture::index)]
    }

    pub fn visible_distortion(&self) -> impl Iterator<Item = Sample> + '_ {
        self.timed_particles(None)
            .map(|p| sample(p.particle, p.age))
    }

    pub fn has_visible_distortion(&self) -> bool {
        self.timed_particles(None).next().is_some()
    }

    pub fn textures(&self) -> Vec<Texture> {
        let mut textures: Vec<_> = [Texture::Flame, Texture::Electric, Texture::Smoke]
            .into_iter()
            .filter(|t| self.first_texture[t.index()] != u64::MAX)
            .collect();
        textures.sort_unstable_by_key(|t| self.first_texture[t.index()]);
        textures
    }
}

pub fn color_at(colormap: &[[f32; 4]], fraction: f32) -> [f32; 4] {
    let count = colormap.len();
    if count == 0 {
        return [0.0; 4];
    }
    let fraction = fraction.clamp(0.0, 1.0);
    if count == 1 || fraction >= 1.0 {
        return colormap[count - 1];
    }
    let position = fraction * (count as f32 - 1.0);
    let index = position.floor() as usize;
    let blend = position - index as f32;
    let inverse = 1.0 - blend;
    let low = colormap[index];
    let high = colormap[(index + 1).min(count - 1)];
    [
        low[0] * inverse + high[0] * blend,
        low[1] * inverse + high[1] * blend,
        low[2] * inverse + high[2] * blend,
        low[3] * inverse + high[3] * blend,
    ]
}

fn sample(particle: &Particle, age: f32) -> Sample {
    sample_at(particle, age, particle.colormap)
}

pub(super) fn sample_with_colors(particle: &Particle, colormap: &[[f32; 4]]) -> Sample {
    sample_at(particle, particle.age, colormap)
}

fn sample_at(particle: &Particle, age: f32, colormap: &[[f32; 4]]) -> Sample {
    let fraction = ((age - particle.delay) / particle.life).clamp(0.0, 1.0);
    let travel = attribute(fraction, particle.speed_exp);
    let growth = signed_attribute(fraction, particle.size_exp);
    let spin = signed_attribute(fraction, particle.rotation_exp);
    Sample {
        x: particle.x + particle.direction_x * travel + particle.force_x * fraction,
        y: particle.y + particle.direction_y * travel + particle.force_y * fraction,
        z: particle.z + particle.direction_z * travel + particle.force_z * fraction,
        size: particle.size + particle.size_end * growth,
        rotation: particle.rotation + particle.rotation_end * spin,
        animation_x: particle.animation_x,
        animation_y: particle.animation_y,
        color: color_at(colormap, fraction),
        distortion_strength: particle.distortion_strength * (1.0 - fraction),
        distortion_scale: particle.distortion_scale,
        distortion_heat: particle.distortion_heat,
    }
}

fn attribute(fraction: f32, exponent: f32) -> f32 {
    let remaining = 1.0 - fraction;
    let exponent = exponent.abs();
    // Preserve the two original roundings for the linear curve. Returning
    // fraction directly would change small values, despite algebraic equality.
    1.0 - if exponent == 1.0 {
        remaining
    } else {
        remaining.powf(exponent)
    }
}

fn signed_attribute(fraction: f32, exponent: f32) -> f32 {
    let value = attribute(fraction, exponent);
    if exponent < 0.0 { 1.0 - value } else { value }
}

#[cfg(test)]
mod tests {
    use super::*;
    const COLORS: &[[f32; 4]] = &[[1.0; 4], [0.0; 4]];
    fn particle(id: i32, kind: usize) -> Particle {
        Particle {
            x: id as f32,
            y: 0.0,
            z: 0.0,
            direction_x: 1.0,
            direction_y: 0.0,
            direction_z: 0.0,
            force_x: 0.0,
            force_y: 0.0,
            force_z: 0.0,
            age: 0.0,
            birth: 0.0,
            death: 0.0,
            effect: 0,
            delay: (id % 7) as f32,
            life: (5 + id % 16) as f32,
            size: 1.0,
            size_end: 2.0,
            rotation: 0.0,
            rotation_end: 1.0,
            speed_exp: 1.0,
            size_exp: 1.0,
            rotation_exp: 1.0,
            animation_x: 0.0,
            animation_y: 0.0,
            distortion: kind == 3,
            distortion_strength: 1.0,
            distortion_scale: 1.0,
            distortion_heat: 1.0,
            colormap: COLORS,
            palette: 0,
            serial: 0,
            texture: [
                Texture::Flame,
                Texture::Electric,
                Texture::Smoke,
                Texture::Flame,
            ][kind],
        }
    }

    #[test]
    fn effect_expiry_matches_per_particle_reference() {
        let mut field = Field::default();
        let mut reference: Vec<Particle> = Vec::new();
        let mut serial = 0;
        for frame in 0..300 {
            let step = if frame % 11 == 0 { 3.0 } else { 1.0 };
            field.advance(step);
            reference.retain_mut(|p| {
                p.age += step;
                p.age <= p.delay + p.life
            });
            for kind in 0..4 {
                let count = 1 + frame % 5;
                let effects: Vec<_> = (0..count)
                    .map(|_| {
                        serial += 1;
                        particle(serial, kind)
                    })
                    .collect();
                // Vary deadlines so expired emissions can sit behind a longer
                // lived one in the allocation queue, as in the LUPS ring.
                let deadline = 26.0 + (frame % 4) as f32 * 10.0;
                reference.extend_from_slice(&effects);
                field.emit(effects.into_iter(), deadline);
            }
            for kind in 0..4 {
                let texture = [
                    Some(Texture::Flame),
                    Some(Texture::Electric),
                    Some(Texture::Smoke),
                    None,
                ][kind];
                let got: Vec<_> = field
                    .timed_particles(texture)
                    .map(|p| (p.x, p.age, p.delay, p.life))
                    .collect();
                let expected: Vec<_> = reference
                    .iter()
                    .filter(|p| {
                        (if p.distortion { 3 } else { p.texture.index() }) == kind
                            && p.age > p.delay
                    })
                    .map(|p| (p.x, p.age, p.delay, p.life))
                    .collect();
                assert_eq!(got, expected, "frame={frame}, group={kind}");
            }
        }
        field.advance(1000.0);
        assert!(field.particles.iter().all(VecDeque::is_empty));
        assert!(field.lifetimes.iter().all(VecDeque::is_empty));
        assert!(field.textures().is_empty());
    }
}
