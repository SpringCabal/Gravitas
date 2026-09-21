use alloc::string::String;
use core::cell::RefCell;

use spring::gl;

use crate::state;

use super::particles::Sample;

const NOISE_TEXTURE: &str = "bitmaps/GPL/Lups/mynoise.png";
const DISTORTION_VERTEX: &str = include_str!("../shaders/fire_distortion.vert");
const DISTORTION_FRAGMENT: &str = include_str!("../shaders/fire_distortion.frag");
const POST_FRAGMENT: &str = include_str!("../shaders/fire_distortion_post.frag");

struct DistortionState {
    width: i32,
    height: i32,
    screen_texture: String,
    distortion_texture: String,
    distortion_shader: u32,
    post_shader: u32,
    distortion_time: i32,
    camera_projection: [f32; 16],
    camera_modelview: [f32; 16],
}

struct SharedState(RefCell<DistortionState>);

const MAX_DISTORTION_SIZE: f32 = 96.0;

// The UI module is single-threaded in the engine, but the host callback API
// requires callback state to be held in a static. This is the same pattern as
// the other WASM post-processing widgets.
unsafe impl Sync for SharedState {}

static STATE: SharedState = SharedState(RefCell::new(DistortionState {
    width: 0,
    height: 0,
    screen_texture: String::new(),
    distortion_texture: String::new(),
    distortion_shader: 0,
    post_shader: 0,
    distortion_time: -1,
    camera_projection: [0.0; 16],
    camera_modelview: [0.0; 16],
}));

pub fn capture_camera() {
    let (Ok(projection), Ok(modelview)) = (
        spring::gfx::get_matrix_data(gl::PROJECTION),
        spring::gfx::get_matrix_data(gl::MODELVIEW),
    ) else {
        return;
    };
    if projection.len() != 16 || modelview.len() != 16 {
        return;
    }
    STATE
        .0
        .borrow_mut()
        .camera_projection
        .copy_from_slice(&projection);
    STATE
        .0
        .borrow_mut()
        .camera_modelview
        .copy_from_slice(&modelview);
}

pub fn draw_screen_effects(width: i32, height: i32) {
    if width <= 1 || height <= 1 {
        return;
    }
    let has_particles = state::with_state(|ui| ui.fire.has_visible_distortion()).unwrap_or(false);
    if !has_particles {
        return;
    }

    ensure_resources(width, height);
    let (screen_texture, distortion_texture, distortion_shader, post_shader) =
        STATE.0.borrow().to_render_values();
    if screen_texture.is_empty()
        || distortion_texture.is_empty()
        || distortion_shader == 0
        || post_shader == 0
    {
        return;
    }

    if spring::gfx::copy_to_texture(
        &screen_texture,
        0,
        0,
        0,
        0,
        width,
        height,
        gl::TEXTURE_2D,
        0,
    )
    .is_err()
    {
        return;
    }

    let _ = spring::gfx::push_attrib(gl::ALL_ATTRIB_BITS);
    let _ = spring::gfx::alpha_test(false, gl::ALWAYS, 0.0);
    let _ = spring::gfx::depth_test(spring::gfx::GfxDepthTestOptions {
        enable: false,
        set_func: false,
        func: 0,
    });
    let _ = spring::gfx::depth_mask(false);
    let _ = spring::gfx::blending(false);
    let _ = spring::gfx::matrix_mode(gl::TEXTURE);
    let _ = spring::gfx::load_identity(0);
    let _ = spring::gfx::matrix_mode(gl::PROJECTION);
    let _ = spring::gfx::push_matrix(0);
    let _ = spring::gfx::load_identity(0);
    let _ = spring::gfx::matrix_mode(gl::MODELVIEW);
    let _ = spring::gfx::push_matrix(0);
    let _ = spring::gfx::load_identity(0);

    let _ = spring::gfx::use_shader(distortion_shader);
    let _ = spring::gfx::bind_texture(NOISE_TEXTURE, 0, true);
    let _ = spring::gfx::render_to_texture(&distortion_texture, render_distortion);
    let _ = spring::gfx::bind_texture(NOISE_TEXTURE, 0, false);

    let _ = spring::gfx::use_shader(post_shader);
    let _ = spring::gfx::bind_texture(&distortion_texture, 0, true);
    let _ = spring::gfx::bind_texture(&screen_texture, 1, true);
    let _ = spring::gfx::tex_rect(-1.0, 1.0, 1.0, -1.0, 0.0, 1.0, 1.0, 0.0);
    let _ = spring::gfx::bind_texture(&screen_texture, 1, false);
    let _ = spring::gfx::bind_texture(&distortion_texture, 0, false);
    let _ = spring::gfx::use_shader(0);

    let _ = spring::gfx::matrix_mode(gl::PROJECTION);
    let _ = spring::gfx::pop_matrix(0);
    let _ = spring::gfx::matrix_mode(gl::MODELVIEW);
    let _ = spring::gfx::pop_matrix(0);
    let _ = spring::gfx::pop_attrib(0);
}

fn ensure_resources(width: i32, height: i32) {
    let mut state = STATE.0.borrow_mut();
    if state.width != width || state.height != height {
        if !state.screen_texture.is_empty() {
            let _ = spring::gfx::delete_texture(&state.screen_texture);
        }
        if !state.distortion_texture.is_empty() {
            let _ = spring::gfx::delete_texture_fbo(&state.distortion_texture);
        }
        state.screen_texture.clear();
        state.distortion_texture.clear();
        state.width = width;
        state.height = height;

        if let Ok(texture) = spring::gfx::create_texture(
            width,
            height,
            0,
            spring::gfx::TextureCreateParams {
                target: gl::TEXTURE_2D,
                min_filter: gl::LINEAR,
                mag_filter: gl::LINEAR,
                wrap_s: gl::CLAMP,
                wrap_t: gl::CLAMP,
                ..Default::default()
            },
        ) {
            state.screen_texture = texture;
        }

        // The displacement is encoded around 0.5 in an ordinary RGBA target,
        // so this works on the same hardware as the rest of the WASM effects.
        if let Ok(texture) = spring::gfx::create_texture(
            width,
            height,
            0,
            spring::gfx::TextureCreateParams {
                target: gl::TEXTURE_2D,
                min_filter: gl::LINEAR,
                mag_filter: gl::LINEAR,
                wrap_s: gl::CLAMP,
                wrap_t: gl::CLAMP,
                fbo: true,
                ..Default::default()
            },
        ) {
            state.distortion_texture = texture;
        }
    }

    if state.distortion_shader == 0
        && let Ok(shader) = spring::gfx::create_shader(spring::gfx::ShaderCreateParams {
            vertex: DISTORTION_VERTEX,
            fragment: DISTORTION_FRAGMENT,
            ..Default::default()
        })
    {
        state.distortion_shader = shader.shader_id;
        state.distortion_time =
            spring::gfx::get_uniform_location(shader.shader_id, "time").unwrap_or(-1);
        let _ = spring::gfx::use_shader(shader.shader_id);
        if let Ok(location) = spring::gfx::get_uniform_location(shader.shader_id, "noiseMap") {
            let _ = spring::gfx::uniform_int(location, &[0], 1);
        }
        let _ = spring::gfx::use_shader(0);
    }

    if state.post_shader == 0
        && let Ok(shader) = spring::gfx::create_shader(spring::gfx::ShaderCreateParams {
            fragment: POST_FRAGMENT,
            ..Default::default()
        })
    {
        state.post_shader = shader.shader_id;
        let _ = spring::gfx::use_shader(shader.shader_id);
        if let Ok(location) = spring::gfx::get_uniform_location(shader.shader_id, "infoTex") {
            let _ = spring::gfx::uniform_int(location, &[0], 1);
        }
        if let Ok(location) = spring::gfx::get_uniform_location(shader.shader_id, "screenTex") {
            let _ = spring::gfx::uniform_int(location, &[1], 1);
        }
        let _ = spring::gfx::use_shader(0);
    }
}

fn render_distortion() {
    let (projection, modelview, time, time_location) = {
        let state = STATE.0.borrow();
        (
            state.camera_projection,
            state.camera_modelview,
            state::with_state(|ui| ui.game_frame as f32).unwrap_or(0.0),
            state.distortion_time,
        )
    };
    if time_location >= 0 {
        let _ = spring::gfx::uniform(time_location, &[time], 1);
    }
    let _ = spring::gfx::clear(gl::COLOR_BUFFER_BIT, &[0.5, 0.5, 0.0, 0.0], 4);
    let _ = spring::gfx::matrix_mode(gl::PROJECTION);
    let _ = spring::gfx::load_matrix(&projection);
    let _ = spring::gfx::matrix_mode(gl::MODELVIEW);
    let _ = spring::gfx::load_matrix(&modelview);
    let _ = spring::gfx::begin_end(gl::QUADS, emit_distortion_vertices);
}

fn emit_distortion_vertices() {
    let Ok(vectors) = spring::unit_rendering::get_camera_vectors(0) else {
        return;
    };
    let _ = state::with_state(|ui| {
        for particle in ui.fire.visible_distortion() {
            distortion_quad(&particle, vectors.up, vectors.right);
        }
    });
}

fn distortion_quad(particle: &Sample, up: spring::types::Float3, right: spring::types::Float3) {
    let radians = particle.rotation * core::f32::consts::PI / 180.0;
    let (sine, cosine) = (radians.sin(), radians.cos());
    // The source LUPS effect grows large world-space particles, but using
    // their full size for a screen-wide post-process quad makes one flame
    // smear the map far beyond its visible sprite. Keep the visible particle
    // size and limit only the distortion footprint.
    let half = particle.size.min(MAX_DISTORTION_SIZE) * 0.5;
    let across = axis(right, cosine, up, sine, half);
    let along = axis(right, -sine, up, cosine, half);
    distortion_corner(particle, 0.0, 0.0, -1.0, -1.0, across, along);
    distortion_corner(particle, 1.0, 0.0, 1.0, -1.0, across, along);
    distortion_corner(particle, 1.0, 1.0, 1.0, 1.0, across, along);
    distortion_corner(particle, 0.0, 1.0, -1.0, 1.0, across, along);
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

fn distortion_corner(
    particle: &Sample,
    u: f32,
    v: f32,
    across_sign: f32,
    along_sign: f32,
    across: spring::types::Float3,
    along: spring::types::Float3,
) {
    let _ = spring::gfx::tex_coord(u, v, 0.0, 0.0, 2);
    let _ = spring::gfx::multi_tex_coord(1, u * 2.0 - 1.0, v * 2.0 - 1.0, 0.0, 0.0, 2);
    let _ = spring::gfx::multi_tex_coord(
        2,
        // JitterParticles2 passes strength * 0.01 to its shader.
        particle.distortion_strength * 0.01,
        particle.distortion_heat,
        particle.distortion_scale,
        0.0,
        4,
    );
    let _ = spring::gfx::vertex(
        particle.x + across.x * across_sign + along.x * along_sign,
        particle.y + across.y * across_sign + along.y * along_sign,
        particle.z + across.z * across_sign + along.z * along_sign,
        1.0,
        3,
    );
}

trait RenderValues {
    fn to_render_values(&self) -> (String, String, u32, u32);
}

impl RenderValues for DistortionState {
    fn to_render_values(&self) -> (String, String, u32, u32) {
        (
            self.screen_texture.clone(),
            self.distortion_texture.clone(),
            self.distortion_shader,
            self.post_shader,
        )
    }
}
