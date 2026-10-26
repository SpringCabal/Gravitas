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

pub struct Field {
    particles: [Vec<Particle>; 4],
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
    pub fn spawn(&mut self, mut particle: Particle) {
        self.serial += 1;
        particle.serial = self.serial;
        let texture = particle.texture.index();
        self.first_texture[texture] = self.first_texture[texture].min(particle.serial);
        let group = if particle.distortion { 3 } else { texture };
        let mut offset = 0;
        for colors in &self.palettes {
            if core::ptr::eq(*colors, particle.colormap) {
                particle.palette = offset;
                self.particles[group].push(particle);
                return;
            }
            offset += colors.len();
        }
        self.palettes.push(particle.colormap);
        self.colors
            .extend_from_slice(particle.colormap.as_flattened());
        particle.palette = offset;
        self.particles[group].push(particle);
    }

    pub fn advance(&mut self, frames: f32) {
        self.elapsed += frames;
        let mut first = [u64::MAX; 3];
        for group in &mut self.particles {
            group.retain_mut(|particle| {
                particle.age += frames;
                let keep = particle.age <= particle.delay + particle.life;
                if keep {
                    let index = particle.texture.index();
                    first[index] = first[index].min(particle.serial);
                }
                keep
            });
        }
        self.first_texture = first;
    }

    pub(super) fn drawable(&self, texture: Texture) -> impl Iterator<Item = &Particle> {
        self.particles[texture.index()]
            .iter()
            .filter(|particle| particle.age > particle.delay)
    }
    pub fn visible(&self, texture: Texture) -> impl Iterator<Item = Sample> {
        self.drawable(texture).map(sample)
    }

    pub(super) fn gpu_particles(&self, texture: Option<Texture>) -> &[Particle] {
        &self.particles[texture.map_or(3, Texture::index)]
    }

    pub fn visible_distortion(&self) -> impl Iterator<Item = Sample> + '_ {
        self.particles[3]
            .iter()
            .filter(|particle| particle.age > particle.delay)
            .map(sample)
    }

    pub fn has_visible_distortion(&self) -> bool {
        self.particles[3]
            .iter()
            .any(|particle| particle.age > particle.delay)
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

fn sample(particle: &Particle) -> Sample {
    sample_with_colors(particle, particle.colormap)
}

pub(super) fn sample_with_colors(particle: &Particle, colormap: &[[f32; 4]]) -> Sample {
    let fraction = ((particle.age - particle.delay) / particle.life).clamp(0.0, 1.0);
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
