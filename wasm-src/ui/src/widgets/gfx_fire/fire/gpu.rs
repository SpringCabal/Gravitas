//! LUPS-style particle curves and billboard expansion on the GPU.
use alloc::collections::VecDeque;
use alloc::vec::Vec;
use core::cell::{Cell, RefCell};
use spring::{ResultExt, gl};

use super::particles::{Field, Particle, Texture};

const STRIDE: usize = 28;
const VERTEX: &str = include_str!("../../../shaders/fire_instanced.vert");
const FRAGMENT: &str = r#"#version 330 compatibility
uniform sampler2D tex0;
void main() { gl_FragColor = texture2D(tex0, gl_TexCoord[0].st) * gl_Color; }
"#;
const DISTORTION: &str = concat!(
    "#version 330 compatibility\n",
    include_str!("../../../shaders/fire_distortion.frag")
);

#[derive(Default)]
pub struct Gpu {
    cpu: Cell<Option<bool>>,
    failed: Cell<bool>,
    data: RefCell<Vec<f32>>,
    resources: RefCell<Option<Resources>>,
}

struct Shader {
    id: u32,
    colors: i32,
    time: i32,
    inverse_view: i32,
    age_frame: i32,
    color_count: usize,
}

impl Shader {
    fn create(spring: &spring::Spring<'_, spring::Ui>, distortion: bool) -> spring::Result<Self> {
        let id = spring
            .gfx()
            .create_shader(
                "",
                VERTEX,
                "",
                "",
                "",
                if distortion { DISTORTION } else { FRAGMENT },
                "",
                Default::default(),
            )?
            .shader_id;
        spring.gfx().use_shader(id)?;
        for (name, value) in [("tex0", 0), ("noiseMap", 0), ("depthTex", 1)] {
            if let Ok(location) = spring.gfx().get_uniform_location(id, name) {
                spring.gfx().uniform_int(location, &[value], 1)?;
            }
        }
        spring.gfx().uniform_int(
            spring.gfx().get_uniform_location(id, "distortion")?,
            &[i32::from(distortion)],
            1,
        )?;
        spring.gfx().use_shader(0)?;
        Ok(Self {
            id,
            colors: spring
                .gfx()
                .get_uniform_location(id, "colors")
                .unwrap_or(-1),
            time: spring.gfx().get_uniform_location(id, "time").unwrap_or(-1),
            age_frame: spring.gfx().get_uniform_location(id, "ageFrame")?,
            color_count: 0,
            inverse_view: spring
                .gfx()
                .get_uniform_location(id, "inverseView")
                .unwrap_or(-1),
        })
    }
}

struct Packet {
    start: usize,
    count: usize,
    death: f32,
    effect: u64,
}

#[derive(Default)]
struct Stream {
    vbo: u32,
    vao: u32,
    capacity: usize,
    head: usize,
    last_serial: u64,
    packets: VecDeque<Packet>,
}

impl Stream {
    fn configure(
        &mut self,
        spring: &spring::Spring<'_, spring::Ui>,
        quad: u32,
        capacity: usize,
    ) -> spring::Result<()> {
        let vbo = spring.gfx().get_vbo(gl::ARRAY_BUFFER, true)?.vbo_id;
        let vao = match spring.gfx().get_vao() {
            Ok(vao) => vao.vao_id,
            Err(error) => {
                let _ = spring.gfx().delete_vbo(vbo);
                return Err(error);
            }
        };
        let attributes = [1, 2, 3, 4, 5, 6, 7].map(|id| spring::gfx::GfxVBOAttributeOptions {
            id,
            size: 4,
            type_: gl::FLOAT,
            normalized: false,
        });
        let result = spring
            .gfx()
            .define_vbo(vbo, capacity as i32, false, 0, false, 0, &attributes)
            .and_then(|()| spring.gfx().attach_vertex_buffer_vao(vao, quad))
            .and_then(|()| spring.gfx().attach_instance_buffer_vao(vao, vbo));
        if let Err(error) = result {
            let _ = spring.gfx().delete_vao(vao);
            let _ = spring.gfx().delete_vbo(vbo);
            return Err(error);
        }
        self.shutdown(spring);
        self.vbo = vbo;
        self.vao = vao;
        self.capacity = capacity;
        Ok(())
    }

    fn prepare(
        &mut self,
        spring: &spring::Spring<'_, spring::Ui>,
        quad: u32,
        field: &Field,
        texture: Option<Texture>,
        data: &mut Vec<f32>,
    ) -> spring::Result<()> {
        while self
            .packets
            .front()
            .is_some_and(|p| p.death < field.elapsed)
        {
            self.packets.pop_front();
        }
        data.clear();
        let mut packets = Vec::new();
        let particles = field.gpu_particles(texture);
        let first = particles.partition_point(|p| p.serial <= self.last_serial);
        for p in particles
            .iter()
            .skip(first)
            .filter(|p| p.death >= field.elapsed)
        {
            append(data, &mut packets, p);
        }
        self.last_serial = field.serial;
        let mut count = data.len() / STRIDE;
        if count == 0 {
            return Ok(());
        }
        let mut start = self.head;
        let tail = self.packets.front().map(|p| p.start);
        let room = match tail {
            None => {
                self.head = 0;
                start = 0;
                count <= self.capacity
            }
            Some(tail) if start > tail => {
                if start + count <= self.capacity {
                    true
                } else if count <= tail {
                    start = 0;
                    true
                } else {
                    false
                }
            }
            Some(tail) => start < tail && start + count <= tail,
        };
        if !room {
            // Growth is infrequent. Repack the live field once; regular draws
            // upload only newly emitted particles into the circular stream.
            data.clear();
            packets.clear();
            for p in field
                .gpu_particles(texture)
                .iter()
                .filter(|p| p.death >= field.elapsed)
            {
                append(data, &mut packets, p);
            }
            count = data.len() / STRIDE;
            self.configure(
                spring,
                quad,
                self.capacity.max(count).max(256).next_power_of_two(),
            )?;
            self.last_serial = field.serial;
            start = 0;
        }
        spring
            .gfx()
            .upload_vbo(self.vbo, data, -1, start as i32, 1, 0)?;
        for mut packet in packets {
            packet.start += start;
            self.packets.push_back(packet);
        }
        self.head = start + count;
        Ok(())
    }

    fn draw(&self, spring: &spring::Spring<'_, spring::Ui>, elapsed: f32) -> spring::Result<()> {
        let mut start = None;
        let mut finish = 0;
        for packet in self.packets.iter().filter(|p| p.death >= elapsed) {
            if let Some(at) = start
                && packet.start < finish
            {
                spring.gfx().draw_arrays_vao(
                    self.vao,
                    gl::TRIANGLES,
                    6,
                    0,
                    (finish - at) as i32,
                    at as i32,
                )?;
                start = None;
            }
            start = Some(start.unwrap_or(packet.start));
            finish = packet.start + packet.count;
        }
        if let Some(at) = start {
            spring.gfx().draw_arrays_vao(
                self.vao,
                gl::TRIANGLES,
                6,
                0,
                (finish - at) as i32,
                at as i32,
            )?;
        }
        Ok(())
    }

    fn shutdown(&mut self, spring: &spring::Spring<'_, spring::Ui>) {
        if self.vao != 0 {
            let _ = spring.gfx().delete_vao(self.vao);
        }
        if self.vbo != 0 {
            let _ = spring.gfx().delete_vbo(self.vbo);
        }
        *self = Self::default();
    }
}

struct Resources {
    shaders: [Shader; 2],
    quad: u32,
    streams: [Stream; 4],
    epoch: u32,
}

impl Resources {
    fn create(spring: &spring::Spring<'_, spring::Ui>) -> spring::Result<Self> {
        let normal = Shader::create(spring, false)?;
        let distortion = match Shader::create(spring, true) {
            Ok(shader) => shader,
            Err(error) => {
                let _ = spring.gfx().delete_shader(normal.id);
                return Err(error);
            }
        };
        let mut result = Self {
            shaders: [normal, distortion],
            quad: 0,
            streams: Default::default(),
            epoch: 0,
        };
        let configure = (|| {
            result.quad = spring.gfx().get_vbo(gl::ARRAY_BUFFER, false)?.vbo_id;
            spring.gfx().define_vbo(
                result.quad,
                6,
                false,
                0,
                false,
                0,
                &[spring::gfx::GfxVBOAttributeOptions {
                    id: 0,
                    size: 2,
                    type_: gl::FLOAT,
                    normalized: false,
                }],
            )?;
            spring.gfx().upload_vbo(
                result.quad,
                &[0., 0., 1., 0., 1., 1., 0., 0., 1., 1., 0., 1.],
                -1,
                0,
                1,
                0,
            )
        })();
        if let Err(error) = configure {
            result.shutdown(spring);
            return Err(error);
        }
        Ok(result)
    }

    fn draw(
        &mut self,
        spring: &spring::Spring<'_, spring::Ui>,
        field: &Field,
        data: &mut Vec<f32>,
        texture: Option<Texture>,
        time: f32,
        inverse_view: [f32; 2],
    ) -> spring::Result<()> {
        if self.epoch != field.epoch {
            for stream in &mut self.streams {
                stream.shutdown(spring);
            }
            for shader in &mut self.shaders {
                shader.color_count = 0;
            }
            self.epoch = field.epoch;
        }
        let index = match texture {
            Some(Texture::Flame) => 0,
            Some(Texture::Electric) => 1,
            Some(Texture::Smoke) => 2,
            None => 3,
        };
        let stream = &mut self.streams[index];
        stream.prepare(spring, self.quad, field, texture, data)?;
        let shader = &mut self.shaders[usize::from(texture.is_none())];
        spring.gfx().use_shader(shader.id)?;
        if shader.colors >= 0 && shader.color_count != field.colors.len() {
            spring
                .gfx()
                .uniform_array_float(shader.colors, &field.colors, 4)?;
            shader.color_count = field.colors.len();
        }
        spring
            .gfx()
            .uniform(shader.age_frame, &[field.elapsed], 1)?;
        if shader.time >= 0 {
            spring.gfx().uniform(shader.time, &[time], 1)?;
        }
        if shader.inverse_view >= 0 {
            spring
                .gfx()
                .uniform(shader.inverse_view, &inverse_view, 2)?;
        }
        let result = stream.draw(spring, field.elapsed);
        spring.gfx().use_shader(0)?;
        result
    }

    fn shutdown(mut self, spring: &spring::Spring<'_, spring::Ui>) {
        for stream in &mut self.streams {
            stream.shutdown(spring);
        }
        if self.quad != 0 {
            let _ = spring.gfx().delete_vbo(self.quad);
        }
        for shader in self.shaders {
            let _ = spring.gfx().delete_shader(shader.id);
        }
    }
}

impl Gpu {
    /// False selects the CPU reference or a logged compatible fallback.
    pub fn draw(
        &self,
        spring: &spring::Spring<'_, spring::Ui>,
        field: &Field,
        texture: Option<Texture>,
        time: f32,
        inverse_view: [f32; 2],
    ) -> bool {
        let cpu = self.cpu.get().unwrap_or_else(|| {
            let cpu = spring
                .game()
                .get_mod_option("fire_renderer")
                .is_ok_and(|v| v.value == "cpu");
            self.cpu.set(Some(cpu));
            spring::log_notice!(
                spring,
                "FireRenderer",
                "{}",
                if cpu { "cpu" } else { "gpu" }
            );
            cpu
        });
        if cpu || self.failed.get() {
            return false;
        }
        if field.gpu_particles(texture).is_empty() {
            return true;
        }
        let mut data = self.data.borrow_mut();
        // All production effects share fewer than 128 colors. Reject overflow
        // explicitly if new effects exceed the shader's uniform budget.
        if field.colors.len() > 128 * 4 {
            spring::log_warning!(
                spring,
                "FireRenderer",
                "palette exceeds 128 colors; using CPU renderer"
            );
            self.failed.set(true);
            return false;
        }
        let mut resources = self.resources.borrow_mut();
        if resources.is_none() {
            *resources =
                Resources::create(spring).log_err_once(spring, "creating GPU fire buffers");
        }
        if let Some(resources) = resources.as_mut()
            && resources
                .draw(spring, field, &mut data, texture, time, inverse_view)
                .log_err_once(spring, "drawing GPU fire buffers")
                .is_some()
        {
            return true;
        }
        let _ = spring.gfx().use_shader(0);
        if let Some(resources) = resources.take() {
            resources.shutdown(spring);
        }
        self.failed.set(true);
        false
    }

    pub fn shutdown(&self, spring: &spring::Spring<'_, spring::Ui>) {
        if let Some(resources) = self.resources.borrow_mut().take() {
            resources.shutdown(spring);
        }
        self.data.borrow_mut().clear();
        self.cpu.set(None);
        self.failed.set(false);
    }
}

fn append(data: &mut Vec<f32>, packets: &mut Vec<Packet>, p: &Particle) {
    let start = data.len() / STRIDE;
    pack(data, p);
    if let Some(packet) = packets.last_mut()
        && packet.effect == p.effect
    {
        packet.count += 1;
    } else {
        packets.push(Packet {
            start,
            count: 1,
            death: p.death,
            effect: p.effect,
        });
    }
}

fn pack(data: &mut Vec<f32>, p: &Particle) {
    data.extend_from_slice(&[
        p.x,
        p.y,
        p.z,
        p.birth,
        p.direction_x,
        p.direction_y,
        p.direction_z,
        p.life,
        p.force_x,
        p.force_y,
        p.force_z,
        p.delay,
        p.size,
        p.size_end,
        p.rotation * (core::f32::consts::PI / 180.0),
        p.rotation_end * (core::f32::consts::PI / 180.0),
        p.speed_exp,
        p.size_exp,
        p.rotation_exp,
        p.palette as f32,
        p.animation_x,
        p.animation_y,
        p.distortion_strength,
        p.distortion_heat,
        p.distortion_scale,
        p.colormap.len().saturating_sub(1) as f32,
        1., // LUPS force exponent
        1., // LUPS distortion animation speed
    ]);
}
