use alloc::string::String;
use core::cell::RefCell;

use spring::gl;

use super::particles::{Field, Sample};

const NOISE_TEXTURE: &str = "bitmaps/GPL/Lups/mynoise.png";
const DISTORTION_VERTEX: &str = include_str!("../shaders/fire_distortion.vert");
const DISTORTION_FRAGMENT: &str = include_str!("../shaders/fire_distortion.frag");
const POST_FRAGMENT: &str = include_str!("../shaders/fire_distortion_post.frag");

struct DistortionState {
    width: i32,
    height: i32,
    screen_texture: String,
    depth_texture: String,
    distortion_texture: String,
    distortion_shader: u32,
    post_shader: u32,
    distortion_time: i32,
    camera_projection: [f32; 16],
    camera_modelview: [f32; 16],
}

/// The heat haze over the flames: its textures, shaders and the camera it
/// was drawn with. Owned by the fire widget.
pub struct Distortion(RefCell<DistortionState>);

impl Default for Distortion {
    fn default() -> Self {
        Self(RefCell::new(DistortionState {
            width: 0,
            height: 0,
            screen_texture: String::new(),
            depth_texture: String::new(),
            distortion_texture: String::new(),
            distortion_shader: 0,
            post_shader: 0,
            distortion_time: -1,
            camera_projection: [0.0; 16],
            camera_modelview: [0.0; 16],
        }))
    }
}

impl Distortion {
    pub fn capture_camera(&self) {
        let (Ok(projection), Ok(modelview)) = (
            spring::gfx::get_matrix_data(gl::PROJECTION),
            spring::gfx::get_matrix_data(gl::MODELVIEW),
        ) else {
            return;
        };
        if projection.len() != 16 || modelview.len() != 16 {
            return;
        }
        let mut state = self.0.borrow_mut();
        state.camera_projection.copy_from_slice(&projection);
        state.camera_modelview.copy_from_slice(&modelview);
    }

    pub fn draw_screen_effects(&self, field: &Field, time: f32, width: i32, height: i32) {
        if width <= 1 || height <= 1 {
            return;
        }
        if !field.has_visible_distortion() {
            return;
        }

        self.ensure_resources(width, height);
        let (screen_texture, distortion_texture, distortion_shader, post_shader) =
            self.0.borrow().to_render_values();
        let depth_texture = self.0.borrow().depth_texture.clone();
        if screen_texture.is_empty()
            || depth_texture.is_empty()
            || distortion_texture.is_empty()
            || distortion_shader == 0
            || post_shader == 0
        {
            return;
        }

        // LUPS tests heat particles against a copy of the scene depth. Sampling
        // it in the shader gives the same occlusion without a general FBO API.
        if spring::gfx::copy_to_texture(
            &depth_texture,
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
        // LUPS accumulates every JitterParticles2 layer into the distortion
        // texture.  Overwriting here made whichever flame happened to be drawn
        // last win, producing hard, repetitive-looking patches instead of the
        // softer combined heat field from the Lua version.
        let _ = spring::gfx::blending(true);
        let _ = spring::gfx::blend_func(gl::ONE, gl::ONE);
        let _ = spring::gfx::matrix_mode(gl::TEXTURE);
        let _ = spring::gfx::push_matrix(0);
        let _ = spring::gfx::load_identity(0);
        let _ = spring::gfx::matrix_mode(gl::PROJECTION);
        let _ = spring::gfx::push_matrix(0);
        let _ = spring::gfx::load_identity(0);
        let _ = spring::gfx::matrix_mode(gl::MODELVIEW);
        let _ = spring::gfx::push_matrix(0);
        let _ = spring::gfx::load_identity(0);

        let _ = spring::gfx::use_shader(distortion_shader);
        let _ = spring::gfx::bind_texture(NOISE_TEXTURE, 0, true);
        let _ = spring::gfx::bind_texture(&depth_texture, 1, true);
        if let Ok(location) = spring::gfx::get_uniform_location(distortion_shader, "inverseView") {
            let _ = spring::gfx::uniform(location, &[1.0 / width as f32, 1.0 / height as f32], 2);
        }
        let _ = spring::gfx::render_to_texture(&distortion_texture, || {
            self.render_distortion(field, time)
        });
        let _ = spring::gfx::bind_texture(&depth_texture, 1, false);
        let _ = spring::gfx::bind_texture(NOISE_TEXTURE, 0, false);

        // The post pass replaces the screen with the sampled scene.  Only the
        // intermediate distortion layers use additive blending.
        let _ = spring::gfx::blending(false);
        let _ = spring::gfx::use_shader(post_shader);
        let _ = spring::gfx::bind_texture(&distortion_texture, 0, true);
        let _ = spring::gfx::bind_texture(&screen_texture, 1, true);
        let _ = spring::gfx::tex_rect(-1.0, 1.0, 1.0, -1.0, 0.0, 1.0, 1.0, 0.0);
        let _ = spring::gfx::bind_texture(&screen_texture, 1, false);
        let _ = spring::gfx::bind_texture(&distortion_texture, 0, false);
        let _ = spring::gfx::use_shader(0);

        let _ = spring::gfx::matrix_mode(gl::TEXTURE);
        let _ = spring::gfx::pop_matrix(0);
        let _ = spring::gfx::matrix_mode(gl::PROJECTION);
        let _ = spring::gfx::pop_matrix(0);
        let _ = spring::gfx::matrix_mode(gl::MODELVIEW);
        let _ = spring::gfx::pop_matrix(0);
        let _ = spring::gfx::pop_attrib(0);
    }

    fn ensure_resources(&self, width: i32, height: i32) {
        let mut state = self.0.borrow_mut();
        if state.width != width || state.height != height {
            if !state.screen_texture.is_empty() {
                let _ = spring::gfx::delete_texture(&state.screen_texture);
            }
            if !state.depth_texture.is_empty() {
                let _ = spring::gfx::delete_texture(&state.depth_texture);
            }
            if !state.distortion_texture.is_empty() {
                let _ = spring::gfx::delete_texture_fbo(&state.distortion_texture);
            }
            state.screen_texture.clear();
            state.depth_texture.clear();
            state.distortion_texture.clear();
            state.width = width;
            state.height = height;

            if let Ok(texture) = spring::gfx::create_texture(
                width,
                height,
                0,
                spring::gfx::TextureCreateParams {
                    target: gl::TEXTURE_2D,
                    format: gl::DEPTH_COMPONENT24,
                    min_filter: gl::NEAREST,
                    mag_filter: gl::NEAREST,
                    wrap_s: gl::CLAMP_TO_EDGE,
                    wrap_t: gl::CLAMP_TO_EDGE,
                    ..Default::default()
                },
            ) {
                state.depth_texture = texture;
            }

            if let Ok(texture) = spring::gfx::create_texture(
                width,
                height,
                0,
                spring::gfx::TextureCreateParams {
                    target: gl::TEXTURE_2D,
                    min_filter: gl::LINEAR,
                    mag_filter: gl::LINEAR,
                    wrap_s: gl::CLAMP_TO_EDGE,
                    wrap_t: gl::CLAMP_TO_EDGE,
                    ..Default::default()
                },
            ) {
                state.screen_texture = texture;
            }

            // Lua's distortionFBO uses a floating-point, nearest-filtered target.
            // Keeping the displacement unquantized avoids hard 8-bit steps in the
            // post-process while nearest filtering preserves each jitter cell.
            if let Ok(texture) = spring::gfx::create_texture(
                width,
                height,
                0,
                spring::gfx::TextureCreateParams {
                    target: gl::TEXTURE_2D,
                    format: gl::RGBA16F,
                    min_filter: gl::NEAREST,
                    mag_filter: gl::NEAREST,
                    wrap_s: gl::CLAMP_TO_EDGE,
                    wrap_t: gl::CLAMP_TO_EDGE,
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
            if let Ok(location) = spring::gfx::get_uniform_location(shader.shader_id, "depthTex") {
                let _ = spring::gfx::uniform_int(location, &[1], 1);
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

    fn render_distortion(&self, field: &Field, time: f32) {
        let (projection, modelview, time_location) = {
            let state = self.0.borrow();
            (
                state.camera_projection,
                state.camera_modelview,
                state.distortion_time,
            )
        };
        if time_location >= 0 {
            let _ = spring::gfx::uniform(time_location, &[time], 1);
        }
        // Distortion stores signed screen-space offsets directly, as the Lua
        // distortionFBO does.  Zero is therefore the neutral value; the old
        // 0.5-centred encoding prevented correct additive accumulation.
        let _ = spring::gfx::clear(gl::COLOR_BUFFER_BIT, &[0.0, 0.0, 0.0, 0.0], 4);
        let _ = spring::gfx::matrix_mode(gl::PROJECTION);
        let _ = spring::gfx::load_matrix(&projection);
        let _ = spring::gfx::matrix_mode(gl::MODELVIEW);
        let _ = spring::gfx::load_matrix(&modelview);
        let _ = spring::gfx::begin_end(gl::QUADS, || emit_distortion_vertices(field));
    }
}

fn emit_distortion_vertices(field: &Field) {
    let Ok(vectors) = spring::unit_rendering::get_camera_vectors(0) else {
        return;
    };
    for particle in field.visible_distortion() {
        distortion_quad(&particle, vectors.up, vectors.right);
    }
}

fn distortion_quad(particle: &Sample, up: spring::types::Float3, right: spring::types::Float3) {
    let radians = particle.rotation * core::f32::consts::PI / 180.0;
    let (sine, cosine) = (radians.sin(), radians.cos());
    let half = particle.size * 0.5;
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
    let _ = spring::gfx::multi_tex_coord(0, u, v, particle.animation_x, particle.animation_y, 4);
    let _ = spring::gfx::multi_tex_coord(1, u * 2.0 - 1.0, v * 2.0 - 1.0, 0.0, 0.0, 2);
    let _ = spring::gfx::multi_tex_coord(
        2,
        // JitterParticles2 scales strength by 0.01 in the vertex shader.
        particle.distortion_strength,
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
