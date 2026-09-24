mod distortion;
mod effects;
mod emit;
mod particles;
#[cfg(feature = "fire-probe")]
pub mod probe;
mod render;

pub use distortion::Distortion;
pub use emit::{burn, electric_field, electric_spark, flame};
pub use render::draw;

use particles::Field;

/// The particles, and a count of emissions that seeds each one.
#[derive(Default)]
pub struct Fire {
    field: Field,
    emissions: u64,
}

impl Fire {
    /// Age the particles by `frames` game frames.
    pub fn advance(&mut self, frames: f32) {
        self.field.advance(frames);
    }
}
