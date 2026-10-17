use spring::gl;

use super::bars::{self, BarStack};

pub fn draw(spring: &spring::Spring<'_, spring::Ui>, seconds: f32) {
    let camera = match spring.camera().get_camera_position() {
        Ok(camera) => camera,
        Err(_) => return,
    };
    let smooth = spring
        .terrain()
        .get_smooth_mesh_height(camera.x, camera.z)
        .unwrap_or(0.0);
    let height = camera.y - smooth;
    if height * height >= MAX_UNIT_DISTANCE {
        return;
    }
    let units = spring
        .unit_rendering()
        .get_visible_units(-1, 0.0, false)
        .unwrap_or_default();
    if units.is_empty() {
        return;
    }
    let blink = (seconds % 1.0) < 0.5;
    let frame = spring.game().current_frame().unwrap_or(0);
    begin(spring);
    for unit_id in units {
        draw_unit(spring, unit_id, camera, blink, frame);
    }
    end(spring);
}

const MAX_UNIT_DISTANCE: f32 = 11_000_000.0;
const MINIMUM_RELOAD_TIME: f32 = 4.0;
const HEIGHT_SCALE: f32 = 1.8;
const FULL_HEALTH_THRESHOLD: f32 = 0.999;

fn begin(spring: &spring::Spring<'_, spring::Ui>) {
    let _ = spring.gfx().push_attrib(gl::ALL_ATTRIB_BITS);
    let _ = spring.gfx().blending(true);
    let _ = spring
        .gfx()
        .blend_func(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
    let _ = spring.gfx().depth_test(spring::gfx::GfxDepthTestOptions {
        enable: true,
        set_func: true,
        func: gl::LEQUAL,
    });
    let _ = spring.gfx().depth_mask(true);
}

fn end(spring: &spring::Spring<'_, spring::Ui>) {
    let _ = spring.gfx().pop_attrib();
}

fn draw_unit(
    spring: &spring::Spring<'_, spring::Ui>,
    unit_id: i32,
    camera: spring::Float3,
    blink: bool,
    frame: i32,
) {
    let Ok(position) = spring.units_info().get_unit_position(
        unit_id,
        spring::units_info::GetUnitPositionOptions {
            mid_pos: false,
            aim_pos: false,
        },
    ) else {
        return;
    };
    let dx = position.x - camera.x;
    let dy = position.y - camera.y;
    let dz = position.z - camera.z;
    if dx * dx + dy * dy + dz * dz > MAX_UNIT_DISTANCE {
        return;
    }
    let stack = collect(spring, unit_id, blink, frame);
    if stack.is_empty() {
        return;
    }
    let radius = spring.units_info().get_unit_radius(unit_id).unwrap_or(0.0);
    let _ = spring.gfx().push_matrix();
    let _ = spring
        .gfx()
        .translate(position.x, position.y + HEIGHT_SCALE * radius, position.z);
    let _ = spring.gfx().billboard();
    for (offset, bar) in stack.iter() {
        draw_bar(spring, offset, bar.progress, bar.color);
    }
    let _ = spring.gfx().pop_matrix();
}

fn draw_bar(spring: &spring::Spring<'_, spring::Ui>, offset: f32, progress: f32, color: [f32; 4]) {
    let _ = spring.gfx().begin_end(gl::QUADS, |spring| {
        quad(
            spring,
            -BAR_WIDTH - OUTLINE_SIZE,
            offset - OUTLINE_SIZE,
            BAR_WIDTH + OUTLINE_SIZE,
            offset + BAR_HEIGHT + OUTLINE_SIZE,
            OUTLINE_BOTTOM,
        );
        quad(
            spring,
            -BAR_WIDTH,
            offset,
            BAR_WIDTH,
            offset + BAR_HEIGHT,
            INNER_BOTTOM,
        );
        quad(
            spring,
            -BAR_WIDTH,
            offset,
            -BAR_WIDTH + 2.0 * BAR_WIDTH * progress,
            offset + BAR_HEIGHT,
            color,
        );
    });
}

fn quad(
    spring: &spring::Spring<'_, spring::Ui>,
    x1: f32,
    y1: f32,
    x2: f32,
    y2: f32,
    color: [f32; 4],
) {
    let _ = spring.gfx().color(color[0], color[1], color[2], color[3]);
    let _ = spring.gfx().vertex(x1, y1, 0.0, 1.0, 1);
    let _ = spring.gfx().vertex(x2, y1, 0.0, 1.0, 1);
    let _ = spring.gfx().vertex(x2, y2, 0.0, 1.0, 1);
    let _ = spring.gfx().vertex(x1, y2, 0.0, 1.0, 1);
}

const BAR_HEIGHT: f32 = 2.3;
const BAR_WIDTH: f32 = 11.5;
const OUTLINE_SIZE: f32 = 0.7;
const OUTLINE_BOTTOM: [f32; 4] = [0.25, 0.25, 0.25, 0.8];
const INNER_BOTTOM: [f32; 4] = [0.06, 0.06, 0.06, 0.5];

fn collect(
    spring: &spring::Spring<'_, spring::Ui>,
    unit_id: i32,
    blink: bool,
    frame: i32,
) -> BarStack {
    let mut stack = BarStack::new();
    if !is_healthbar_target(spring, unit_id) {
        return stack;
    }
    let Ok(health) = spring.units_info().get_unit_health(unit_id) else {
        return stack;
    };
    let max_health = health.max_health.max(1.0);
    let build = health.build_progress;
    let fraction = health.health / max_health;
    if fraction >= FULL_HEALTH_THRESHOLD {
        return stack;
    }
    let paralyze = health.paralyze_damage / max_health;
    if build >= 1.0 || build - fraction >= 0.01 {
        stack.push(fraction, bars::health_color(fraction));
    }
    if build < 1.0 {
        stack.push(build, bars::BUILD);
    }
    if paralyze > 0.01 && fraction > 0.01 && paralyze < 1e8 {
        let stunned = spring
            .units_info()
            .get_unit_is_stunned(unit_id)
            .unwrap_or(false);
        let color = if stunned {
            if blink {
                bars::EMP_BLINK
            } else {
                bars::EMP_PAUSED
            }
        } else {
            bars::EMP
        };
        let progress = if stunned { 1.0 } else { paralyze.min(1.0) };
        stack.push(progress, color);
    }
    if health.capture_progress > 0.0 {
        stack.push(health.capture_progress, bars::CAPTURE);
    }
    push_reload(spring, &mut stack, unit_id, frame);
    stack
}

fn is_healthbar_target(spring: &spring::Spring<'_, spring::Ui>, unit_id: i32) -> bool {
    spring
        .units_info()
        .get_unit_def_id(unit_id)
        .is_ok_and(|def_id| crate::defs::unit(spring, def_id).healthbar)
}

fn push_reload(
    spring: &spring::Spring<'_, spring::Ui>,
    stack: &mut BarStack,
    unit_id: i32,
    frame: i32,
) {
    let Ok(state) = spring
        .units_weapons()
        .get_unit_weapon_state(unit_id, 1, "reloadTime")
    else {
        return;
    };
    let reload_time = state.reload_time;
    if reload_time < MINIMUM_RELOAD_TIME {
        return;
    }
    let reload_frame = state.reload_frame;
    if reload_frame <= frame as f32 {
        return;
    }
    let progress =
        1.0 - ((reload_frame - frame as f32) * spring::consts::FRAME_SECONDS) / reload_time;
    stack.push(progress.max(0.0), bars::RELOAD);
}
