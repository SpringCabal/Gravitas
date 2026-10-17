use core::cell::Cell;

use gravitas_shared::Input as InputMessage;
use spring::Float3;
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
    pub fn poll(&self, spring: &spring::Spring<'_, spring::Ui>, global: &Global) {
        let Some(player_unit) = global.player_unit.get() else {
            return;
        };
        let Some(pointer) = pointer(spring, global) else {
            return;
        };
        let held = self.jump.get() || pointer.middle;
        let was_held = self.jump_held.replace(held);
        let (move_x, move_z) = self.movement.axis(spring);
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
                springcabal::message::send_to_rules(spring, &input.encode())
            });
    }

    pub fn key(
        &self,
        spring: &spring::Spring<'_, spring::Ui>,
        event: &KeyEvent<'_>,
        down: bool,
    ) -> bool {
        if event.matches("e") || event.matches("space") {
            self.jump.set(down);
            return true;
        }
        self.movement.set(spring, event, down);
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

fn pointer(spring: &spring::Spring<'_, spring::Ui>, global: &Global) -> Option<Pointer> {
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
    let mouse = spring.input().get_mouse_state().ok()?;
    let (target, target_unit) = trace(spring, mouse.x, mouse.y);
    Some(Pointer {
        target,
        target_unit,
        left: mouse.left,
        right: mouse.right,
        middle: mouse.middle,
    })
}

fn trace(spring: &spring::Spring<'_, spring::Ui>, x: f32, y: f32) -> (Float3, i32) {
    let position = spring
        .camera()
        .ground_at(x, y)
        .ok()
        .flatten()
        .unwrap_or_default();
    let target_unit = spring
        .camera()
        .unit_at(x, y)
        .ok()
        .flatten()
        .unwrap_or(NO_UNIT);
    (position, target_unit)
}
