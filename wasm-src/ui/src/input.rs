use alloc::format;
use core::cell::Cell;

use spring::types::Float3;

use crate::state::Global;

/// The keys held and what was last sent to the rules module.
#[derive(Default)]
pub struct Input {
    north: Cell<bool>,
    south: Cell<bool>,
    west: Cell<bool>,
    east: Cell<bool>,
    jump: Cell<bool>,
    /// Whether jump was held at the previous poll.
    jump_held: Cell<bool>,
    last_sent: Cell<Option<Sent>>,
}

/// What one input message says, and for which player unit.
#[derive(Clone, Copy, PartialEq)]
struct Sent {
    player_unit: i32,
    move_x: f32,
    move_z: f32,
    target: Float3,
    target_unit: i32,
    push: bool,
    pull: bool,
    jump: bool,
}

/// Where the cursor points and the mouse buttons held.
struct Pointer {
    target: Float3,
    target_unit: i32,
    left: bool,
    right: bool,
    middle: bool,
}

impl Input {
    /// Send the player's input to the rules module when it changed. A jump is
    /// sent once, when the key or the middle button goes down, as in the Lua
    /// game.
    pub fn poll(&self, global: &Global) {
        let Some(player_unit) = global.player_unit.get() else {
            return;
        };
        let Some(pointer) = pointer(global) else {
            return;
        };
        let held = self.jump.get() || pointer.middle;
        let sent = Sent {
            // The player unit is part of the key: a new unit needs the input
            // again.
            player_unit,
            move_x: f32::from(self.east.get()) - f32::from(self.west.get()),
            move_z: f32::from(self.south.get()) - f32::from(self.north.get()),
            target: pointer.target,
            target_unit: pointer.target_unit,
            push: pointer.left,
            pull: pointer.right,
            jump: held && !self.jump_held.replace(held),
        };
        if self.last_sent.get() == Some(sent) {
            return;
        }
        let message = format!(
            "gravitas|{}|{}|{}|{}|{}|{}|{}|{}|{}",
            sent.move_x,
            sent.move_z,
            sent.target.x,
            sent.target.y,
            sent.target.z,
            sent.target_unit,
            i32::from(sent.push),
            i32::from(sent.pull),
            i32::from(sent.jump),
        );
        match spring::send_lua_rules_msg(&message) {
            Ok(_) => self.last_sent.set(Some(sent)),
            // Not recorded as sent, so the next poll tries again.
            Err(error) => crate::diagnostics::warning(&format!(
                "player input not sent (error {}): {message}",
                error.code
            )),
        }
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
const NO_UNIT: i32 = -1;
const MOUSE_LEFT: i32 = 1;
const MOUSE_RIGHT: i32 = 3;

/// The mouse, or the e2e scenario's scripted pointer when it has one.
fn pointer(global: &Global) -> Option<Pointer> {
    #[cfg(feature = "e2e")]
    if let Some(scripted) = global.scripted_pointer.get() {
        return Some(Pointer {
            target: scripted.target,
            target_unit: NO_UNIT,
            left: false,
            right: false,
            middle: scripted.jump,
        });
    }
    #[cfg(not(feature = "e2e"))]
    let _ = global;
    let mouse = spring::input::get_mouse_state(0).ok()?;
    let (target, target_unit) = trace(mouse.x, mouse.y);
    Some(Pointer {
        target,
        target_unit,
        left: mouse.left,
        right: mouse.right,
        middle: mouse.middle,
    })
}

fn trace(x: f32, y: f32) -> (Float3, i32) {
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
        .unwrap_or(NO_UNIT);
    (position, target_unit)
}
