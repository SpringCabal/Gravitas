mod worker;
use super::geometry::axes;
use super::particles::{Field, Particle, Sample, Texture, sample_with_colors};
use alloc::vec::Vec;
use springcabal::jobs::{self, Reader, put_f32, put_u32};
pub(crate) use worker::worker;

pub(super) fn vertices(
    options: &mut jobs::Options,
    spring: &spring::Spring<'_, spring::Ui>,
    field: &Field,
    texture: Texture,
    up: spring::Float3,
    right: spring::Float3,
    vertices: &mut Vec<f32>,
) -> bool {
    if !options.enabled(spring) {
        return false;
    }
    let particles: Vec<_> = field.drawable(texture).collect();
    if particles.is_empty() {
        return true;
    }
    let mut palettes: Vec<&[[f32; 4]]> = Vec::new();
    let mut indices = Vec::with_capacity(particles.len());
    for p in &particles {
        let index = palettes
            .iter()
            .position(|colors| core::ptr::eq(*colors, p.colormap))
            .unwrap_or_else(|| {
                palettes.push(p.colormap);
                palettes.len() - 1
            });
        indices.push(index);
    }
    // Input carries a palette index and 19 floats per particle, in addition
    // to the camera/palettes header. It is larger than the compact output.
    let header_bytes = 32 + palettes.iter().map(|p| 4 + p.len() * 16).sum::<usize>();
    let max_input_particles = spring::jobs::MAX_BYTES.saturating_sub(header_bytes) / 80;
    let max_particles = max_input_particles.min(spring::jobs::MAX_BYTES / (13 * 4));
    if max_particles == 0 {
        return false;
    }
    let chunk_size = particles.len().div_ceil(jobs::CHUNKS).min(max_particles);
    let mut inputs = Vec::new();
    let mut limits = Vec::new();
    for (chunk, colors) in particles.chunks(chunk_size).zip(indices.chunks(chunk_size)) {
        let mut bytes = Vec::new();
        for v in [up.x, up.y, up.z, right.x, right.y, right.z] {
            put_f32(&mut bytes, v);
        }
        put_u32(&mut bytes, palettes.len() as u32);
        for colors in &palettes {
            put_u32(&mut bytes, colors.len() as u32);
            for color in *colors {
                for value in color {
                    put_f32(&mut bytes, *value);
                }
            }
        }
        put_u32(&mut bytes, chunk.len() as u32);
        for (p, color) in chunk.iter().zip(colors) {
            put_u32(&mut bytes, *color as u32);
            for v in [
                p.x,
                p.y,
                p.z,
                p.direction_x,
                p.direction_y,
                p.direction_z,
                p.force_x,
                p.force_y,
                p.force_z,
                p.age,
                p.delay,
                p.life,
                p.size,
                p.size_end,
                p.rotation,
                p.rotation_end,
                p.speed_exp,
                p.size_exp,
                p.rotation_exp,
            ] {
                put_f32(&mut bytes, v);
            }
        }
        limits.push(chunk.len() * 13 * 4);
        inputs.push(bytes);
    }
    let Some(output) = options.run(
        spring,
        crate::FIRE_VERTICES,
        &inputs,
        &limits,
        "fire-vertices",
        worker,
    ) else {
        return false;
    };
    for bytes in output {
        assert_eq!(bytes.len() % (13 * 4), 0, "invalid fire billboard packet");
        let mut reader = Reader::new(&bytes);
        for _ in 0..bytes.len() / (13 * 4) {
            let sample = Sample {
                x: reader.f32(),
                y: reader.f32(),
                z: reader.f32(),
                color: [reader.f32(), reader.f32(), reader.f32(), reader.f32()],
                size: 0.0,
                rotation: 0.0,
                animation_x: 0.0,
                animation_y: 0.0,
                distortion_strength: 0.0,
                distortion_scale: 0.0,
                distortion_heat: 0.0,
            };
            let across = spring::Float3::new(reader.f32(), reader.f32(), reader.f32());
            let along = spring::Float3::new(reader.f32(), reader.f32(), reader.f32());
            super::geometry::quad_axes(vertices, &sample, across, along);
        }
        reader.finish();
    }
    if options.checking() {
        let mut expected = Vec::new();
        for sample in field.visible(texture) {
            if sample.color[3] > 0.0 {
                super::geometry::quad(&mut expected, &sample, up, right);
            }
        }
        assert_eq!(vertices.len(), expected.len(), "fire job geometry length");
        for (actual, expected) in vertices.iter().zip(expected) {
            assert_eq!(
                actual.to_bits(),
                expected.to_bits(),
                "fire job geometry bits"
            );
        }
    }
    true
}
