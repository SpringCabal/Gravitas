mod distortion;
mod effects;
mod emit;
mod particles;
mod render;

pub use emit::{burn, electric_field, electric_spark, flame, update};
pub use particles::Field;
pub use render::draw;

pub fn draw_screen_effects(width: i32, height: i32) {
    distortion::draw_screen_effects(width, height);
}
