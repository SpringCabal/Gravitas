use spring::gl;

use super::effects::SMOKE_TEXTURE;
use super::geometry::quad;
use super::particles::{Field, Texture};
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
        distortion.draw_screen_effects(
            spring,
            field,
            batch,
            time,
            view.view_size_x,
            view.view_size_y,
        );
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
    let _ = spring.gfx().bind_texture(texture.path(), 0, true);
    if batch.gpu.draw(spring, field, Some(texture), 0.0, [0.0; 2]) {
        let _ = spring.gfx().bind_texture(texture.path(), 0, false);
        return;
    }
    let Ok(vectors) = spring.unit_rendering().get_camera_vectors() else {
        return;
    };
    let (up, right) = (vectors.up, vectors.right);
    let mut vertices = batch.vertices.borrow_mut();
    vertices.clear();
    if !super::jobs::vertices(
        &mut batch.jobs.borrow_mut(),
        spring,
        field,
        texture,
        up,
        right,
        &mut vertices,
    ) {
        for particle in field.visible(texture) {
            if particle.color[3] <= 0.0 {
                continue;
            }
            quad(&mut vertices, &particle, up, right);
        }
    }
    if vertices.is_empty() {
        return;
    }
    let _ = spring.gfx().bind_texture(texture.path(), 0, true);
    batch.draw(spring, &vertices);
    let _ = spring.gfx().bind_texture(texture.path(), 0, false);
}

const ALPHA_FLOOR: f32 = 0.0;
