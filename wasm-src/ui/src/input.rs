use alloc::format;

use crate::state;

pub fn poll() {
    if player_unit().is_none() {
        return;
    }
    let (move_x, move_z) = state::with_state(|ui| ui.move_vector()).unwrap_or((0.0, 0.0));
    let jump = state::with_state(|ui| ui.jump).unwrap_or(false);
    let Ok(mouse) = spring::input::get_mouse_state(0) else {
        return;
    };
    let (target, target_unit) = match scripted_target() {
        Some(target) => (target, NO_UNIT),
        None => trace(mouse.x, mouse.y),
    };
    send(
        move_x,
        move_z,
        target,
        target_unit,
        mouse.left,
        mouse.right,
        jump || mouse.middle,
    );
}

fn send(
    move_x: f32,
    move_z: f32,
    target: spring::types::Float3,
    target_unit: i32,
    push: bool,
    pull: bool,
    jump: bool,
) {
    let _ = spring::send_lua_rules_msg(&format!(
        "gravitas|{move_x}|{move_z}|{}|{}|{}|{target_unit}|{}|{}|{}",
        target.x,
        target.y,
        target.z,
        i32::from(push),
        i32::from(pull),
        i32::from(jump),
    ));
}

pub fn mouse_press(_x: i32, _y: i32, button: i32) -> bool {
    matches!(button, MOUSE_LEFT | MOUSE_RIGHT)
}

pub fn mouse_release(_x: i32, _y: i32, _button: i32) {}

pub fn key_press(key_code: i32) -> bool {
    if key_code == KEY_F11 || key_code == KEY_F12 {
        let _ = spring::send_commands("screenshot", "");
        return true;
    }
    set_key(key_code, true)
}

pub fn key_release(key_code: i32) -> bool {
    set_key(key_code, false)
}

const KEY_W: i32 = 119;
const KEY_A: i32 = 97;
const KEY_S: i32 = 115;
const KEY_D: i32 = 100;
const KEY_E: i32 = 101;
const KEY_SPACE: i32 = 32;
const KEY_F11: i32 = 292;
const KEY_F12: i32 = 293;
const KEY_UP: i32 = 273;
const KEY_DOWN: i32 = 274;
const KEY_RIGHT: i32 = 275;
const KEY_LEFT: i32 = 276;
const HIT_UNIT: i32 = 1;
const NO_UNIT: i32 = -1;
const MOUSE_LEFT: i32 = 1;
const MOUSE_RIGHT: i32 = 3;

fn set_key(key_code: i32, pressed: bool) -> bool {
    state::with_state(|ui| match key_code {
        KEY_W | KEY_UP => {
            ui.move_north = pressed;
            true
        }
        KEY_S | KEY_DOWN => {
            ui.move_south = pressed;
            true
        }
        KEY_A | KEY_LEFT => {
            ui.move_west = pressed;
            true
        }
        KEY_D | KEY_RIGHT => {
            ui.move_east = pressed;
            true
        }
        KEY_E | KEY_SPACE => {
            ui.jump = pressed;
            true
        }
        _ => false,
    })
    .unwrap_or(false)
}

fn scripted_target() -> Option<spring::types::Float3> {
    let (x, z) = state::with_state(|ui| ui.scripted_target).flatten()?;
    Some(spring::types::Float3 {
        x,
        y: spring::get_ground_height(x, z).unwrap_or(0.0),
        z,
    })
}

fn player_unit() -> Option<i32> {
    state::with_state(|ui| ui.player_unit).filter(|unit_id| *unit_id >= 0)
}

fn trace(x: f32, y: f32) -> (spring::types::Float3, i32) {
    let ground = spring::camera::trace_screen_ray(
        x,
        y,
        spring::camera::TraceScreenRayOptions {
            only_coords: true,
            use_minimap: false,
            include_sky: false,
            ignore_water: false,
            height_offset: 0.0,
        },
    );
    let unit = spring::camera::trace_screen_ray(
        x,
        y,
        spring::camera::TraceScreenRayOptions {
            only_coords: false,
            use_minimap: false,
            include_sky: false,
            ignore_water: false,
            height_offset: 0.0,
        },
    );
    let position = ground.map(|value| value.hit_pos).unwrap_or_default();
    let target_unit = unit
        .ok()
        .filter(|value| value.hit_type == HIT_UNIT)
        .map(|value| value.hit_id)
        .unwrap_or(-1);
    (position, target_unit)
}
