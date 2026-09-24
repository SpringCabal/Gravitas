use alloc::vec::Vec;
use core::cell::Cell;

use spring::types::Float3;
use springcabal_control::goal::{Bounds, MoveGoal};

use crate::units::{self, WEAPON_PULL, WEAPON_PUSH};

#[derive(Clone, Copy, Default)]
pub struct PlayerCommand {
    pub move_x: f32,
    pub move_z: f32,
    pub target_x: f32,
    pub target_y: f32,
    pub target_z: f32,
    pub target_unit: i32,
    pub push: bool,
    pub pull: bool,
    /// Jump was pressed: set only on the message sent when the key went down.
    pub jump: bool,
}

#[derive(Default)]
pub struct Player {
    unit_id: Option<i32>,
    latest: Option<PlayerCommand>,
    moved_last: bool,
    target_unit: Option<i32>,
    aiming: bool,
    jump_request: Option<(f32, f32, f32)>,
}

/// What the player's input asks of the player unit this frame.
pub enum Order {
    ClearTarget,
    TargetUnit(i32),
    TargetGround(f32, f32, f32),
    /// Keep a weapon from firing on its own while its button is up.
    HoldWeapon(i32),
    Stop,
    MoveTo(Float3),
}

/// Queue the ui module's latest input for the player gadget. A jump press
/// sticks until the gadget has seen it.
pub fn accept(pending: &Cell<Option<PlayerCommand>>, command: PlayerCommand) {
    let unseen_press = pending.get().is_some_and(|pending| pending.jump);
    pending.set(Some(PlayerCommand {
        jump: command.jump || unseen_press,
        ..command
    }));
}

impl Player {
    pub fn unit_id(&self) -> Option<i32> {
        self.unit_id
    }

    /// Take control of a new player unit. Returns true if it is one.
    pub fn adopt(&mut self, unit_id: i32, unit_def_id: i32, unit_team: i32) -> bool {
        if unit_team != PLAYER_TEAM || crate::defs::get().gravit != Some(unit_def_id) {
            return false;
        }
        self.unit_id = Some(unit_id);
        true
    }

    pub fn release(&mut self, unit_id: i32) {
        if self.unit_id == Some(unit_id) {
            self.unit_id = None;
            self.latest = None;
            self.aiming = false;
            self.target_unit = None;
        }
    }

    /// Turn the latest input into orders. `input` is only taken while there is
    /// a player unit.
    pub fn update(&mut self, input: &Cell<Option<PlayerCommand>>, frame: i32) -> Vec<Order> {
        let mut orders = Vec::new();
        let Some(unit_id) = self.unit_id else {
            return orders;
        };
        if let Some(command) = input.take() {
            if command.jump {
                self.jump_request = Some((command.target_x, command.target_y, command.target_z));
            } else if command.move_x != 0.0 || command.move_z != 0.0 {
                self.jump_request = None;
            }
            self.latest = Some(command);
        }
        let Some(command) = self.latest else {
            return orders;
        };
        self.aim(unit_id, &command, &mut orders);
        if !command.push {
            orders.push(Order::HoldWeapon(WEAPON_PUSH));
        }
        if !command.pull {
            orders.push(Order::HoldWeapon(WEAPON_PULL));
        }
        if frame % MOVE_INTERVAL == 0 {
            self.drive(unit_id, &command, &mut orders);
        }
        orders
    }

    pub fn take_jump_request(&mut self) -> Option<(i32, f32, f32, f32)> {
        let unit_id = self.unit_id?;
        let (x, y, z) = self.jump_request.take()?;
        Some((unit_id, x, y, z))
    }

    pub fn retry_jump(&mut self, x: f32, y: f32, z: f32) {
        self.jump_request = Some((x, y, z));
    }
}

/// Carry out `orders` for the player unit.
pub fn apply(unit_id: i32, orders: Vec<Order>, frame: i32) {
    for order in orders {
        match order {
            Order::ClearTarget => set_target(unit_id, NO_TARGET, Float3::default(), false),
            Order::TargetUnit(target_id) => {
                set_target(unit_id, target_id, Float3::default(), false);
            }
            Order::TargetGround(x, y, z) => {
                set_target(unit_id, NO_TARGET, Float3 { x, y, z }, true)
            }
            Order::HoldWeapon(weapon) => hold_weapon(unit_id, weapon, frame),
            Order::Stop => {
                units::order(unit_id, spring::cmd::STOP, &[]);
            }
            Order::MoveTo(goal) => {
                crate::diagnostics::check(STEER.set(unit_id, goal), || {
                    alloc::format!("move goal for the player unit {unit_id}")
                });
            }
        }
    }
}

/// The player unit holds fire until the player shoots.
pub fn disable_auto_fire(unit_id: i32) {
    units::order(unit_id, spring::cmd::FIRE_STATE, &[FIRE_STATE_HOLD]);
}

const PLAYER_TEAM: i32 = 0;
const MOVE_INTERVAL: i32 = 3;
const FIRE_STATE_HOLD: f32 = 0.0;
/// The held keys send the player a short way ahead, on the ground; a
/// diagonal reaches further. The path finder plans the way there.
const STEER: MoveGoal = MoveGoal::ahead(32.0)
    .unnormalized()
    .radius(0.0)
    .raw(false)
    .bounds(Bounds::Unbounded);
const RELOAD_HOLD_FRAMES: i32 = 2;
const NO_TARGET: i32 = -1;
const BEAM_RANGE: f32 = 420.0;

impl Player {
    fn aim(&mut self, unit_id: i32, command: &PlayerCommand, orders: &mut Vec<Order>) {
        if !command.push && !command.pull {
            if self.aiming {
                orders.push(Order::ClearTarget);
                self.target_unit = None;
                self.aiming = false;
            }
            return;
        }
        self.aiming = true;
        if command.target_unit >= 0
            && let (Some(origin), Some(target)) = (
                units::position(unit_id),
                units::position(command.target_unit),
            )
        {
            let dx = target.x - origin.x;
            let dz = target.z - origin.z;
            if dx * dx + dz * dz <= BEAM_RANGE * BEAM_RANGE {
                if self.target_unit != Some(command.target_unit) {
                    self.target_unit = Some(command.target_unit);
                    orders.push(Order::TargetUnit(command.target_unit));
                }
                return;
            }
            self.target_unit = None;
            let (x, y, z) = clamp_target(origin, target.x, target.y, target.z);
            orders.push(Order::TargetGround(x, y, z));
            return;
        }
        self.target_unit = None;
        let y = if command.target_y.abs() > f32::EPSILON {
            command.target_y
        } else {
            units::ground_height(command.target_x, command.target_z)
        };
        let Some(origin) = units::position(unit_id) else {
            return;
        };
        let (x, y, z) = clamp_target(origin, command.target_x, y, command.target_z);
        orders.push(Order::TargetGround(x, y, z));
    }

    fn drive(&mut self, unit_id: i32, command: &PlayerCommand, orders: &mut Vec<Order>) {
        if command.move_x == 0.0 && command.move_z == 0.0 {
            if self.moved_last {
                orders.push(Order::Stop);
                self.moved_last = false;
            }
            return;
        }
        let Some(position) = units::position(unit_id) else {
            return;
        };
        let Some(goal) = STEER.goal(position, (command.move_x, command.move_z)) else {
            return;
        };
        orders.push(Order::MoveTo(goal));
        self.moved_last = true;
    }
}

fn clamp_target(origin: Float3, x: f32, y: f32, z: f32) -> (f32, f32, f32) {
    let dx = x - origin.x;
    let dz = z - origin.z;
    let distance = (dx * dx + dz * dz).sqrt();
    if distance <= BEAM_RANGE || distance <= f32::EPSILON {
        return (x, y, z);
    }
    let scale = BEAM_RANGE / distance;
    let x = origin.x + dx * scale;
    let z = origin.z + dz * scale;
    (x, units::ground_height(x, z), z)
}

fn hold_weapon(unit_id: i32, weapon: i32, frame: i32) {
    // The read mirrors Spring.GetUnitWeaponState and is one-based.  The
    // native UnitControl setter takes the engine's zero-based weapon index.
    let Ok(state) = spring::units_weapons::get_unit_weapon_state(unit_id, weapon, "reloadFrame")
    else {
        return;
    };
    if frame as f32 >= state.reload_frame {
        let _ = spring::set_unit_weapon_state(
            unit_id,
            weapon - 1,
            "reloadFrame",
            (frame + RELOAD_HOLD_FRAMES) as f32,
        );
    }
}

fn set_target(unit_id: i32, target_id: i32, pos: Float3, is_ground_target: bool) {
    let _ = spring::set_unit_target(
        unit_id,
        spring::types::UnitTargetRef {
            target_id,
            pos,
            is_ground_target,
        },
        spring::types::SetUnitTargetOptions {
            manual_fire: false,
            user_target: true,
        },
        0,
    );
}
