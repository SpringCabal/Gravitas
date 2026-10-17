use spring::gl;

use super::effects::SMOKE_TEXTURE;
use super::particles::{Field, Sample};
use super::{Distortion, Fire};

pub fn draw(
    spring: &spring::Spring<'_, spring::Ui>,
    fire: &Fire,
    distortion: &Distortion,
    time: f32,
) {
    distortion.capture_camera(spring);
    let field = &fire.field;
    let textures = field.textures();
    if textures.is_empty() {
        return;
    }
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
        draw_texture(spring, field, texture);
    }
    if let Ok(view) = spring.display().get_view_geometry() {
        distortion.draw_screen_effects(spring, field, time, view.view_size_x, view.view_size_y);
    }
    if textures.contains(&SMOKE_TEXTURE) {
        draw_texture(spring, field, SMOKE_TEXTURE);
    }
    let _ = spring.gfx().pop_attrib();
}

fn draw_texture(spring: &spring::Spring<'_, spring::Ui>, field: &Field, texture: &'static str) {
    let _ = spring.gfx().bind_texture(texture, 0, true);
    let _ = spring
        .gfx()
        .begin_end(gl::QUADS, |spring| emit_vertices(spring, field, texture));
    let _ = spring.gfx().bind_texture(texture, 0, false);
}

fn emit_vertices(spring: &spring::Spring<'_, spring::Ui>, field: &Field, texture: &'static str) {
    let Ok(vectors) = spring.unit_rendering().get_camera_vectors() else {
        return;
    };
    let (up, right) = (vectors.up, vectors.right);
    for particle in field.visible(texture) {
        if particle.color[3] <= 0.0 {
            continue;
        }
        quad(spring, &particle, up, right);
    }
}

const ALPHA_FLOOR: f32 = 0.0;

fn quad(
    spring: &spring::Spring<'_, spring::Ui>,
    particle: &Sample,
    up: spring::Float3,
    right: spring::Float3,
) {
    let radians = -particle.rotation * core::f32::consts::PI / 180.0;
    let (sine, cosine) = (radians.sin(), radians.cos());
    let half = particle.size * 0.5;
    let across = axis(right, cosine, up, sine, half);
    let along = axis(right, -sine, up, cosine, half);
    let _ = spring.gfx().color(
        particle.color[0],
        particle.color[1],
        particle.color[2],
        particle.color[3],
    );
    corner(spring, particle, 0.0, 0.0, -1.0, -1.0, across, along);
    corner(spring, particle, 1.0, 0.0, 1.0, -1.0, across, along);
    corner(spring, particle, 1.0, 1.0, 1.0, 1.0, across, along);
    corner(spring, particle, 0.0, 1.0, -1.0, 1.0, across, along);
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
    spring: &spring::Spring<'_, spring::Ui>,
    particle: &Sample,
    u: f32,
    v: f32,
    across_sign: f32,
    along_sign: f32,
    across: spring::Float3,
    along: spring::Float3,
) {
    let _ = spring.gfx().tex_coord(u, v, 0.0, 0.0, 2);
    let _ = spring.gfx().vertex(
        particle.x + across.x * across_sign + along.x * along_sign,
        particle.y + across.y * across_sign + along.y * along_sign,
        particle.z + across.z * across_sign + along.z * along_sign,
        1.0,
        3,
    );
}
