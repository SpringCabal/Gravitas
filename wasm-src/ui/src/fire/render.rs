use spring::gl;

use crate::state;

use super::distortion;
use super::effects::SMOKE_TEXTURE;
use super::particles::Sample;

pub fn draw() {
    distortion::capture_camera();
    let textures = state::with_state(|ui| ui.fire.textures()).unwrap_or_default();
    if textures.is_empty() {
        return;
    }
    let _ = spring::gfx::push_attrib(gl::ALL_ATTRIB_BITS);
    let _ = spring::gfx::blending(true);
    let _ = spring::gfx::blend_func(gl::ONE, gl::ONE_MINUS_SRC_ALPHA);
    let _ = spring::gfx::depth_test(spring::gfx::GfxDepthTestOptions {
        enable: true,
        set_func: true,
        func: gl::LEQUAL,
    });
    let _ = spring::gfx::depth_mask(false);
    let _ = spring::gfx::alpha_test(true, gl::GREATER, ALPHA_FLOOR);
    for (index, texture) in textures.iter().enumerate() {
        if *texture == SMOKE_TEXTURE {
            continue;
        }
        let _ = state::with_state(|ui| ui.fire_pass = index);
        let _ = spring::gfx::bind_texture(texture, 0, true);
        let _ = spring::gfx::begin_end(gl::QUADS, emit_vertices);
        let _ = spring::gfx::bind_texture(texture, 0, false);
    }
    // LUPS composites heat after layer 0 flames and before layer 1 smoke.
    if let Ok(view) = spring::display::get_view_geometry(0) {
        distortion::draw_screen_effects(view.view_size_x, view.view_size_y);
    }
    if let Some(index) = textures
        .iter()
        .position(|texture| *texture == SMOKE_TEXTURE)
    {
        let _ = state::with_state(|ui| ui.fire_pass = index);
        let _ = spring::gfx::bind_texture(SMOKE_TEXTURE, 0, true);
        let _ = spring::gfx::begin_end(gl::QUADS, emit_vertices);
        let _ = spring::gfx::bind_texture(SMOKE_TEXTURE, 0, false);
    }
    let _ = spring::gfx::pop_attrib(0);
}

pub fn emit_vertices() {
    let Ok(vectors) = spring::unit_rendering::get_camera_vectors(0) else {
        return;
    };
    let (up, right) = (vectors.up, vectors.right);
    let _ = state::with_state(|ui| {
        let Some(texture) = ui.fire.textures().get(ui.fire_pass).copied() else {
            return;
        };
        for particle in ui.fire.visible(texture) {
            if particle.color[3] <= 0.0 {
                continue;
            }
            quad(&particle, up, right);
        }
    });
}

const ALPHA_FLOOR: f32 = 0.0;

fn quad(particle: &Sample, up: spring::types::Float3, right: spring::types::Float3) {
    // GLSL's mat2(ca, -sa, sa, ca) in SimpleParticles2 is column-major.
    let radians = -particle.rotation * core::f32::consts::PI / 180.0;
    let (sine, cosine) = (radians.sin(), radians.cos());
    let half = particle.size * 0.5;
    let across = axis(right, cosine, up, sine, half);
    let along = axis(right, -sine, up, cosine, half);
    let _ = spring::gfx::color(
        particle.color[0],
        particle.color[1],
        particle.color[2],
        particle.color[3],
    );
    corner(particle, 0.0, 0.0, -1.0, -1.0, across, along);
    corner(particle, 1.0, 0.0, 1.0, -1.0, across, along);
    corner(particle, 1.0, 1.0, 1.0, 1.0, across, along);
    corner(particle, 0.0, 1.0, -1.0, 1.0, across, along);
}

fn axis(
    right: spring::types::Float3,
    right_scale: f32,
    up: spring::types::Float3,
    up_scale: f32,
    half: f32,
) -> spring::types::Float3 {
    spring::types::Float3 {
        x: (right.x * right_scale + up.x * up_scale) * half,
        y: (right.y * right_scale + up.y * up_scale) * half,
        z: (right.z * right_scale + up.z * up_scale) * half,
    }
}

fn corner(
    particle: &Sample,
    u: f32,
    v: f32,
    across_sign: f32,
    along_sign: f32,
    across: spring::types::Float3,
    along: spring::types::Float3,
) {
    let _ = spring::gfx::tex_coord(u, v, 0.0, 0.0, 2);
    let _ = spring::gfx::vertex(
        particle.x + across.x * across_sign + along.x * along_sign,
        particle.y + across.y * across_sign + along.y * along_sign,
        particle.z + across.z * across_sign + along.z * along_sign,
        1.0,
        3,
    );
}
