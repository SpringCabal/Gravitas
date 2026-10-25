mod batch;
mod distortion;
mod effects;
mod emit;
mod geometry;
pub(crate) mod jobs;
mod particles;
#[cfg(feature = "fire-probe")]
pub mod probe;
mod render;

pub use batch::Batch;
pub use distortion::Distortion;
pub use emit::{burn, electric_field, electric_spark, flame};
pub use render::draw;

use particles::Field;

#[derive(Default)]
pub struct Fire {
    field: Field,
    emissions: u64,
}

impl Fire {
    pub fn advance(&mut self, frames: f32) {
        self.field.advance(frames);
    }
}
