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
    pub jump: bool,
}

pub struct Player {
    unit_id: Option<i32>,
    pending: Option<PlayerCommand>,
    latest: Option<PlayerCommand>,
    moved_last: bool,
    target_unit: i32,
    aiming: bool,
    jump_request: Option<(f32, f32, f32)>,
}

impl Player {
    pub fn new() -> Self {
        Self {
            unit_id: None,
            pending: None,
            latest: None,
            moved_last: false,
            target_unit: NO_TARGET,
            aiming: false,
            jump_request: None,
        }
    }

    pub fn unit_id(&self) -> Option<i32> {
        self.unit_id
    }

    pub fn adopt(&mut self, unit_id: i32, unit_def_id: i32, unit_team: i32) {
        if unit_team == 0 && units::def_id(units::GRAVIT) == Some(unit_def_id) {
            self.unit_id = Some(unit_id);
            disable_auto_fire(unit_id);
        }
    }

    pub fn release(&mut self, unit_id: i32) {
        if self.unit_id == Some(unit_id) {
            self.unit_id = None;
            self.latest = None;
            self.aiming = false;
            self.target_unit = NO_TARGET;
        }
    }

    pub fn accept(&mut self, command: PlayerCommand) {
        let held = self.pending.is_some_and(|pending| pending.jump);
        self.pending = Some(PlayerCommand {
            jump: command.jump || held,
            ..command
        });
    }

    pub fn update(&mut self, frame: i32) {
        let Some(unit_id) = self.unit_id else {
            return;
        };
        if let Some(command) = self.pending.take() {
            if command.jump {
                self.jump_request = Some((command.target_x, command.target_y, command.target_z));
            } else if command.move_x != 0.0 || command.move_z != 0.0 {
                self.jump_request = None;
            }
            self.latest = Some(command);
        }
        let Some(command) = self.latest else {
            return;
        };
        self.aim(unit_id, &command);
        self.gate_weapons(unit_id, &command, frame);
        if frame % MOVE_INTERVAL == 0 {
            self.drive(unit_id, &command);
        }
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

const MOVE_INTERVAL: i32 = 3;
const MOVE_REACH: f32 = 32.0;
const RELOAD_HOLD_FRAMES: i32 = 2;
const NO_TARGET: i32 = -1;
const BEAM_RANGE: f32 = 420.0;

impl Player {
    fn aim(&mut self, unit_id: i32, command: &PlayerCommand) {
        if !command.push && !command.pull {
            if self.aiming {
                clear_unit_target(unit_id);
                self.target_unit = NO_TARGET;
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
                if self.target_unit != command.target_unit {
                    self.target_unit = command.target_unit;
                    set_unit_target(unit_id, command.target_unit);
                }
                return;
            }
            self.target_unit = NO_TARGET;
            let (x, y, z) = clamp_target(origin, target.x, target.y, target.z);
            set_ground_target(unit_id, x, y, z);
            return;
        }
        self.target_unit = NO_TARGET;
        let y = if command.target_y.abs() > f32::EPSILON {
            command.target_y
        } else {
            units::ground_height(command.target_x, command.target_z)
        };
        let Some(origin) = units::position(unit_id) else {
            return;
        };
        let (x, y, z) = clamp_target(origin, command.target_x, y, command.target_z);
        set_ground_target(unit_id, x, y, z);
    }

    fn gate_weapons(&self, unit_id: i32, command: &PlayerCommand, frame: i32) {
        hold_weapon(unit_id, WEAPON_PUSH, command.push, frame);
        hold_weapon(unit_id, WEAPON_PULL, command.pull, frame);
    }

    fn drive(&mut self, unit_id: i32, command: &PlayerCommand) {
        if command.move_x == 0.0 && command.move_z == 0.0 {
            if self.moved_last {
                let _ = spring::give_order_to_unit(unit_id, crate::cmd::STOP, &[], 0, 0);
                self.moved_last = false;
            }
            return;
        }
        let Some(position) = units::position(unit_id) else {
            return;
        };
        let x = position.x + MOVE_REACH * command.move_x;
        let z = position.z + MOVE_REACH * command.move_z;
        let y = units::ground_height(x, z);
        let _ =
            spring::set_unit_move_goal(unit_id, spring::types::Float3 { x, y, z }, 0.0, 0.0, false);
        self.moved_last = true;
    }
}

fn clamp_target(origin: spring::types::Float3, x: f32, y: f32, z: f32) -> (f32, f32, f32) {
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

fn disable_auto_fire(unit_id: i32) {
    let _ = spring::give_order_to_unit(
        unit_id,
        crate::cmd::FIRE_STATE,
        &[crate::cmd::FIRE_STATE_HOLD],
        0,
        0,
    );
}

fn hold_weapon(unit_id: i32, weapon: i32, firing: bool, frame: i32) {
    if firing {
        return;
    }
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

fn clear_unit_target(unit_id: i32) {
    let _ = spring::set_unit_target(
        unit_id,
        spring::types::UnitTargetRef {
            target_id: NO_TARGET,
            pos: spring::types::Float3::default(),
            is_ground_target: false,
        },
        spring::types::SetUnitTargetOptions {
            manual_fire: false,
            user_target: true,
        },
        0,
    );
}

fn set_unit_target(unit_id: i32, target_id: i32) {
    let _ = spring::set_unit_target(
        unit_id,
        spring::types::UnitTargetRef {
            target_id,
            pos: spring::types::Float3::default(),
            is_ground_target: false,
        },
        spring::types::SetUnitTargetOptions {
            manual_fire: false,
            user_target: true,
        },
        0,
    );
}

fn set_ground_target(unit_id: i32, x: f32, y: f32, z: f32) {
    let _ = spring::set_unit_target(
        unit_id,
        spring::types::UnitTargetRef {
            target_id: -1,
            pos: spring::types::Float3 { x, y, z },
            is_ground_target: true,
        },
        spring::types::SetUnitTargetOptions {
            manual_fire: false,
            user_target: true,
        },
        0,
    );
}
