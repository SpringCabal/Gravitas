use super::{Particle, Texture, axes, sample_with_colors};
use alloc::vec::Vec;
use springcabal::jobs::{Reader, put_f32};
pub(crate) fn worker(bytes: &[u8]) -> Vec<u8> {
    let mut reader = Reader::new(bytes);
    let up = spring::Float3::new(reader.f32(), reader.f32(), reader.f32());
    let right = spring::Float3::new(reader.f32(), reader.f32(), reader.f32());
    let count = reader.u32();
    let palettes: Vec<Vec<[f32; 4]>> = (0..count)
        .map(|_| {
            let count = reader.u32();
            (0..count)
                .map(|_| [reader.f32(), reader.f32(), reader.f32(), reader.f32()])
                .collect()
        })
        .collect();
    let count = reader.u32();
    let mut output = Vec::with_capacity(count as usize * 13 * 4);
    for _ in 0..count {
        let palette = reader.u32() as usize;
        let particle = Particle {
            x: reader.f32(),
            y: reader.f32(),
            z: reader.f32(),
            direction_x: reader.f32(),
            direction_y: reader.f32(),
            direction_z: reader.f32(),
            force_x: reader.f32(),
            force_y: reader.f32(),
            force_z: reader.f32(),
            age: reader.f32(),
            delay: reader.f32(),
            life: reader.f32(),
            size: reader.f32(),
            size_end: reader.f32(),
            rotation: reader.f32(),
            rotation_end: reader.f32(),
            speed_exp: reader.f32(),
            size_exp: reader.f32(),
            rotation_exp: reader.f32(),
            animation_x: 0.0,
            animation_y: 0.0,
            distortion: false,
            distortion_strength: 0.0,
            distortion_scale: 0.0,
            distortion_heat: 0.0,
            colormap: &[],
            texture: Texture::Flame,
        };
        let sample = sample_with_colors(&particle, &palettes[palette]);
        if sample.color[3] > 0.0 {
            let (across, along) = axes(&sample, up, right);
            for value in [
                sample.x,
                sample.y,
                sample.z,
                sample.color[0],
                sample.color[1],
                sample.color[2],
                sample.color[3],
                across.x,
                across.y,
                across.z,
                along.x,
                along.y,
                along.z,
            ] {
                put_f32(&mut output, value);
            }
        }
    }
    reader.finish();
    output
}
