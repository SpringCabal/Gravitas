const VERTEX_SOURCE: &str = include_str!("shaders/healthbars.vert");

const GLOW_ALPHA: f32 = 0.14;

pub fn create() -> Option<u32> {
    let shader = spring::gfx::create_shader(spring::gfx::ShaderCreateParams {
        vertex: VERTEX_SOURCE,
        ..Default::default()
    })
    .ok()?;
    (shader.shader_id != 0).then(|| {
        apply_glow(shader.shader_id);
        shader.shader_id
    })
}

fn apply_glow(shader_id: u32) {
    let _ = spring::gfx::use_shader(shader_id);
    if let Ok(location) = spring::gfx::get_uniform_location(shader_id, "glowAlpha") {
        let _ = spring::gfx::uniform(location, &[GLOW_ALPHA], 1);
    }
    let _ = spring::gfx::use_shader(0);
}
