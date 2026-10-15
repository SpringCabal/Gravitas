use core::cell::Cell;

use gravitas_shared::Input as InputMessage;
use spring::types::Float3;
use spring_addons::KeyEvent;
use springcabal::keys::MoveKeys;
use springcabal::message::SendOnChange;

use crate::state::Global;

#[derive(Default)]
pub struct Input {
    movement: MoveKeys,
    jump: Cell<bool>,
    jump_held: Cell<bool>,
    sent: SendOnChange<(i32, InputMessage)>,
}

struct Pointer {
    target: Float3,
    target_unit: i32,
    left: bool,
    right: bool,
    middle: bool,
}

impl Input {
    pub fn poll(&self, global: &Global) {
        let Some(player_unit) = global.player_unit.get() else {
            return;
        };
        let Some(pointer) = pointer(global) else {
            return;
        };
        let held = self.jump.get() || pointer.middle;
        let was_held = self.jump_held.replace(held);
        let (move_x, move_z) = self.movement.axis();
        let input = InputMessage {
            move_x: move_x as f32,
            move_z: move_z as f32,
            target: [pointer.target.x, pointer.target.y, pointer.target.z],
            target_unit: pointer.target_unit,
            push: pointer.left,
            pull: pointer.right,
            jump: held && !was_held,
        };
        self.sent
            .send_if_changed((player_unit, input), |(_, input)| {
                springcabal::message::send_to_rules(&input.encode())
            });
    }

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
    matches!(
        button,
        spring::mouse::LEFT | spring::mouse::MIDDLE | spring::mouse::RIGHT
    )
}

pub fn mouse_release(_x: i32, _y: i32, _button: i32) {}

const NO_UNIT: i32 = -1;

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
