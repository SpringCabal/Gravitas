use alloc::vec::Vec;
use core::cell::{Cell, RefCell};
use spring::{ResultExt, gl};

const STRIDE: usize = 9;
const VERTEX: &str = r#"#version 330 compatibility
layout(location = 0) in vec3 position;
layout(location = 1) in vec4 tint;
layout(location = 2) in vec2 uv;
void main() {
    vec4 eye = gl_ModelViewMatrix * vec4(position, 1.0);
    gl_Position = gl_ProjectionMatrix * eye;
    gl_ClipVertex = eye;
    gl_FogFragCoord = abs(eye.z);
    gl_FrontColor = tint;
    gl_BackColor = tint;
    gl_TexCoord[0] = gl_TextureMatrix[0] * vec4(uv, 0.0, 1.0);
}
"#;

/// A reusable packed stream, retaining the fixed-function fragment pipeline.
#[derive(Default)]
pub struct Batch {
    pub gpu: super::gpu::Gpu,
    pub vertices: RefCell<Vec<f32>>,
    pub jobs: RefCell<springcabal::jobs::Options>,
    resources: RefCell<Option<Resources>>,
    failed: Cell<bool>,
}

struct Resources {
    shader: u32,
    vbo: u32,
    vao: u32,
    capacity: usize,
}

impl Resources {
    fn create(spring: &spring::Spring<'_, spring::Ui>) -> spring::Result<Self> {
        let shader = spring
            .gfx()
            .create_shader("", VERTEX, "", "", "", "", "", Default::default())?
            .shader_id;
        let vbo = match spring.gfx().get_vbo(gl::ARRAY_BUFFER, true) {
            Ok(vbo) => vbo.vbo_id,
            Err(error) => {
                let _ = spring.gfx().delete_shader(shader);
                return Err(error);
            }
        };
        let vao = match spring.gfx().get_vao() {
            Ok(vao) => vao.vao_id,
            Err(error) => {
                let _ = spring.gfx().delete_vbo(vbo);
                let _ = spring.gfx().delete_shader(shader);
                return Err(error);
            }
        };
        Ok(Self {
            shader,
            vbo,
            vao,
            capacity: 0,
        })
    }

    fn draw(
        &mut self,
        spring: &spring::Spring<'_, spring::Ui>,
        vertices: &[f32],
    ) -> spring::Result<()> {
        let count = vertices.len() / STRIDE;
        if count > self.capacity {
            let capacity = count.max(64).next_power_of_two();
            let attributes =
                [(0, 3), (1, 4), (2, 2)].map(|(id, size)| spring::gfx::GfxVBOAttributeOptions {
                    id,
                    size,
                    type_: gl::FLOAT,
                    normalized: false,
                });
            // Native buffers are defined once. Grow by replacing the stream;
            // a VAO also accepts each vertex-buffer attachment only once.
            let (vbo, vao) = if self.capacity == 0 {
                (self.vbo, self.vao)
            } else {
                let vbo = spring.gfx().get_vbo(gl::ARRAY_BUFFER, true)?.vbo_id;
                let vao = match spring.gfx().get_vao() {
                    Ok(vao) => vao.vao_id,
                    Err(error) => {
                        let _ = spring.gfx().delete_vbo(vbo);
                        return Err(error);
                    }
                };
                (vbo, vao)
            };
            let configure = spring
                .gfx()
                .define_vbo(vbo, capacity as i32, false, 0, false, 0, &attributes)
                .and_then(|()| spring.gfx().attach_vertex_buffer_vao(vao, vbo));
            if let Err(error) = configure {
                if self.capacity != 0 {
                    let _ = spring.gfx().delete_vao(vao);
                    let _ = spring.gfx().delete_vbo(vbo);
                }
                return Err(error);
            }
            if self.capacity != 0 {
                let _ = spring.gfx().delete_vao(self.vao);
                let _ = spring.gfx().delete_vbo(self.vbo);
                self.vbo = vbo;
                self.vao = vao;
            }
            self.capacity = capacity;
        }
        spring.gfx().upload_vbo(self.vbo, vertices, -1, 0, 1, 0)?;
        spring.gfx().use_shader(self.shader)?;
        let result = spring
            .gfx()
            .draw_arrays_vao(self.vao, gl::TRIANGLES, count as i32, 0, 0, 0);
        spring.gfx().use_shader(0)?;
        result
    }

    fn shutdown(self, spring: &spring::Spring<'_, spring::Ui>) {
        let _ = spring.gfx().delete_vao(self.vao);
        let _ = spring.gfx().delete_vbo(self.vbo);
        let _ = spring.gfx().delete_shader(self.shader);
    }
}

impl Batch {
    pub fn draw(&self, spring: &spring::Spring<'_, spring::Ui>, vertices: &[f32]) {
        if vertices.is_empty() {
            return;
        }
        let mut resources = self.resources.borrow_mut();
        if resources.is_none() && !self.failed.get() {
            *resources =
                Resources::create(spring).log_err_once(spring, "creating packed fire buffers");
            self.failed.set(resources.is_none());
        }
        if let Some(resources) = resources.as_mut()
            && resources
                .draw(spring, vertices)
                .log_err_once(spring, "drawing packed fire buffers")
                .is_some()
        {
            return;
        }
        // A failed stream stays on the compatible path until shutdown, rather
        // than retrying a broken resource every frame.
        if let Some(resources) = resources.take() {
            resources.shutdown(spring);
            self.failed.set(true);
        }
        // Keep the compatible path when buffer creation is unavailable.
        let _ = spring.gfx().use_shader(0);
        spring
            .gfx()
            .begin_end(gl::TRIANGLES, |spring| {
                for v in vertices.chunks_exact(STRIDE) {
                    let _ = spring.gfx().color(v[3], v[4], v[5], v[6]);
                    let _ = spring.gfx().tex_coord(v[7], v[8], 0.0, 0.0, 2);
                    let _ = spring.gfx().vertex(v[0], v[1], v[2], 1.0, 3);
                }
            })
            .log_err_once(spring, "drawing compatible fire quads");
    }

    pub fn shutdown(&self, spring: &spring::Spring<'_, spring::Ui>) {
        self.gpu.shutdown(spring);
        if let Some(resources) = self.resources.borrow_mut().take() {
            resources.shutdown(spring);
        }
        self.vertices.borrow_mut().clear();
        self.failed.set(false);
    }
}
