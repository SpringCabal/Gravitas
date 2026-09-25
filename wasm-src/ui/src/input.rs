use core::cell::Cell;

use gravitas_shared::Input as InputMessage;
use spring::types::Float3;
use spring_addons::KeyEvent;
use springcabal::keys::MoveKeys;
use springcabal::message::SendOnChange;

use crate::state::Global;

/// The keys held and what was last sent to the rules module.
#[derive(Default)]
pub struct Input {
    movement: MoveKeys,
    jump: Cell<bool>,
    /// Whether jump was held at the previous poll.
    jump_held: Cell<bool>,
    /// The player unit is part of it: a new unit needs the input again.
    sent: SendOnChange<(i32, InputMessage)>,
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
        let (move_x, move_z) = self.movement.axis();
        let input = InputMessage {
            move_x: move_x as f32,
            move_z: move_z as f32,
            target: [pointer.target.x, pointer.target.y, pointer.target.z],
            target_unit: pointer.target_unit,
            push: pointer.left,
            pull: pointer.right,
            jump: held && !self.jump_held.replace(held),
        };
        // A failed send isn't recorded, so the next poll tries again.
        self.sent
            .send_if_changed((player_unit, input), |(_, input)| {
                let message = input.encode();
                spring::send_lua_rules_msg(&message)
                    .map_err(|error| {
                        crate::diagnostics::warning(&alloc::format!(
                            "player input not sent (error {}): {message}",
                            error.code
                        ))
                    })
                    .is_ok()
            });
    }

    /// A key went down or up; whether it is one of the game's.
    pub fn key(&self, event: &KeyEvent<'_>, down: bool) -> bool {
        if event.matches("e") || event.matches("space") {
            self.jump.set(down);
            return true;
        }
        self.movement.set(event, down);
        MoveKeys::is_move_key(event)
    }
}

pub fn mouse_press(_x: i32, _y: i32, button: i32) -> bool {
    matches!(button, spring::mouse::LEFT | spring::mouse::RIGHT)
}

pub fn mouse_release(_x: i32, _y: i32, _button: i32) {}

const NO_UNIT: i32 = -1;

/// The mouse, or the e2e scenario's scripted pointer when it has one.
fn pointer(global: &Global) -> Option<Pointer> {
    #[cfg(feature = "headless-integration")]
    if let Some(scripted) = global.scripted_pointer.get() {
        return Some(Pointer {
            target: scripted.target,
            target_unit: NO_UNIT,
            left: false,
            right: false,
            middle: scripted.jump,
        });
    }
    #[cfg(not(feature = "headless-integration"))]
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
    let position = spring::pick::ground_at(x, y).unwrap_or_default();
    let target_unit = spring::pick::unit_at(x, y).unwrap_or(NO_UNIT);
    (position, target_unit)
}
