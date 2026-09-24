use alloc::format;
use alloc::string::String;
use core::cell::{Cell, RefCell};

use crate::state::Global;

/// The keys held and what was last sent to the rules module.
#[derive(Default)]
pub struct Input {
    north: Cell<bool>,
    south: Cell<bool>,
    west: Cell<bool>,
    east: Cell<bool>,
    jump: Cell<bool>,
    last_sent: RefCell<String>,
}

impl Input {
    /// Send the player's input to the rules module when it changed. A held
    /// jump is re-sent every frame: the rules retry it after each landing.
    pub fn poll(&self, global: &Global) {
        let Some(player_unit) = global.player_unit.get() else {
            return;
        };
        let move_x = f32::from(self.east.get()) - f32::from(self.west.get());
        let move_z = f32::from(self.south.get()) - f32::from(self.north.get());
        let Ok(mouse) = spring::input::get_mouse_state(0) else {
            return;
        };
        let (target, target_unit) = trace(mouse.x, mouse.y);
        let jump = self.jump.get() || mouse.middle;
        let message = format!(
            "gravitas|{move_x}|{move_z}|{}|{}|{}|{target_unit}|{}|{}|{}",
            target.x,
            target.y,
            target.z,
            i32::from(mouse.left),
            i32::from(mouse.right),
            i32::from(jump),
        );
        let mut last_sent = self.last_sent.borrow_mut();
        // The player unit is part of the key: a new unit needs the input again.
        let key = format!("{player_unit}|{message}");
        if !jump && *last_sent == key {
            return;
        }
        *last_sent = key;
        let _ = spring::send_lua_rules_msg(&message);
    }

    pub fn key_press(&self, key_code: i32) -> bool {
        self.set_key(key_code, true)
    }

    pub fn key_release(&self, key_code: i32) -> bool {
        self.set_key(key_code, false)
    }

    fn set_key(&self, key_code: i32, pressed: bool) -> bool {
        let key = match key_code {
            KEY_W | KEY_UP => &self.north,
            KEY_S | KEY_DOWN => &self.south,
            KEY_A | KEY_LEFT => &self.west,
            KEY_D | KEY_RIGHT => &self.east,
            KEY_E | KEY_SPACE => &self.jump,
            _ => return false,
        };
        key.set(pressed);
        true
    }
}

pub fn mouse_press(_x: i32, _y: i32, button: i32) -> bool {
    matches!(button, MOUSE_LEFT | MOUSE_RIGHT)
}

pub fn mouse_release(_x: i32, _y: i32, _button: i32) {}

const KEY_W: i32 = 119;
const KEY_A: i32 = 97;
const KEY_S: i32 = 115;
const KEY_D: i32 = 100;
const KEY_E: i32 = 101;
const KEY_SPACE: i32 = 32;
const KEY_UP: i32 = 273;
const KEY_DOWN: i32 = 274;
const KEY_RIGHT: i32 = 275;
const KEY_LEFT: i32 = 276;
const HIT_UNIT: i32 = 1;
const MOUSE_LEFT: i32 = 1;
const MOUSE_RIGHT: i32 = 3;

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
