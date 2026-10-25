use alloc::vec::Vec;
use spring::gl;

use super::effects::SMOKE_TEXTURE;
use super::particles::{Field, Sample, Texture};
use super::{Batch, Distortion, Fire};

pub fn draw(
    spring: &spring::Spring<'_, spring::Ui>,
    fire: &Fire,
    distortion: &Distortion,
    batch: &Batch,
    time: f32,
) {
    let field = &fire.field;
    let textures = field.textures();
    if textures.is_empty() {
        return;
    }
    distortion.capture_camera(spring);
    let _ = spring.gfx().push_attrib(gl::ALL_ATTRIB_BITS);
    let _ = spring.gfx().blending(true);
    let _ = spring.gfx().blend_func(gl::ONE, gl::ONE_MINUS_SRC_ALPHA);
    let _ = spring.gfx().depth_test(spring::gfx::GfxDepthTestOptions {
        enable: true,
        set_func: true,
        func: gl::LEQUAL,
    });
    let _ = spring.gfx().depth_mask(false);
    let _ = spring.gfx().alpha_test(true, gl::GREATER, ALPHA_FLOOR);
    for texture in &textures {
        if *texture == SMOKE_TEXTURE {
            continue;
        }
        draw_texture(spring, field, *texture, batch);
    }
    if let Ok(view) = spring.display().get_view_geometry() {
        distortion.draw_screen_effects(spring, field, time, view.view_size_x, view.view_size_y);
    }
    if textures.contains(&SMOKE_TEXTURE) {
        draw_texture(spring, field, SMOKE_TEXTURE, batch);
    }
    let _ = spring.gfx().pop_attrib();
}

fn draw_texture(
    spring: &spring::Spring<'_, spring::Ui>,
    field: &Field,
    texture: Texture,
    batch: &Batch,
) {
    let Ok(vectors) = spring.unit_rendering().get_camera_vectors() else {
        return;
    };
    let (up, right) = (vectors.up, vectors.right);
    let mut vertices = batch.vertices.borrow_mut();
    vertices.clear();
    for particle in field.visible(texture) {
        if particle.color[3] <= 0.0 {
            continue;
        }
        quad(&mut vertices, &particle, up, right);
    }
    if vertices.is_empty() {
        return;
    }
    let _ = spring.gfx().bind_texture(texture.path(), 0, true);
    batch.draw(spring, &vertices);
    let _ = spring.gfx().bind_texture(texture.path(), 0, false);
}

const ALPHA_FLOOR: f32 = 0.0;

fn quad(vertices: &mut Vec<f32>, particle: &Sample, up: spring::Float3, right: spring::Float3) {
    let radians = -particle.rotation * core::f32::consts::PI / 180.0;
    let (sine, cosine) = (radians.sin(), radians.cos());
    let half = particle.size * 0.5;
    let across = axis(right, cosine, up, sine, half);
    let along = axis(right, -sine, up, cosine, half);
    for (u, v, a, b) in [
        (0.0, 0.0, -1.0, -1.0),
        (1.0, 0.0, 1.0, -1.0),
        (1.0, 1.0, 1.0, 1.0),
        (0.0, 0.0, -1.0, -1.0),
        (1.0, 1.0, 1.0, 1.0),
        (0.0, 1.0, -1.0, 1.0),
    ] {
        corner(vertices, particle, u, v, a, b, across, along);
    }
}

fn axis(
    right: spring::Float3,
    right_scale: f32,
    up: spring::Float3,
    up_scale: f32,
    half: f32,
) -> spring::Float3 {
    spring::Float3 {
        x: (right.x * right_scale + up.x * up_scale) * half,
        y: (right.y * right_scale + up.y * up_scale) * half,
        z: (right.z * right_scale + up.z * up_scale) * half,
    }
}

fn corner(
    vertices: &mut Vec<f32>,
    particle: &Sample,
    u: f32,
    v: f32,
    across_sign: f32,
    along_sign: f32,
    across: spring::Float3,
    along: spring::Float3,
) {
    vertices.extend_from_slice(&[
        particle.x + across.x * across_sign + along.x * along_sign,
        particle.y + across.y * across_sign + along.y * along_sign,
        particle.z + across.z * across_sign + along.z * along_sign,
    ]);
    vertices.extend_from_slice(&particle.color);
    vertices.extend_from_slice(&[u, v]);
}
