use core::cell::Cell;

use crate::state::Global;

/// The overhead camera that follows the player.
#[derive(Default)]
pub struct Camera {
    /// Updates that have set the first camera state so far.
    initial_updates: Cell<u8>,
}

#[cfg(feature = "e2e")]
pub fn look_at(global: &Global, x: f32, z: f32, height: Option<f32>) {
    global.scripted.access_mut().camera = Some((x, z, height.unwrap_or(HEIGHT)));
}

impl Camera {
    pub fn follow(&self, global: &Global) {
        let Some((position, velocity, transition, height)) = target(global) else {
            return;
        };
        let initial = self.initial_updates.get() < INITIAL_CAMERA_UPDATES;
        let speed = game_speed();
        let camera = spring::CameraState {
            name: "ta",
            position: spring::types::Float3 {
                x: position.x + LEAD * speed * velocity.x,
                y: 0.0,
                z: position.z + LEAD * speed * velocity.z,
            },
            direction: spring::types::Float3 {
                x: 0.0,
                y: PITCH_Y,
                z: PITCH_Z,
            },
            up: spring::types::Float3 {
                x: 0.0,
                y: -PITCH_Z,
                z: PITCH_Y,
            },
            right: spring::types::Float3 {
                x: 1.0,
                y: 0.0,
                z: 0.0,
            },
            fov: FIELD_OF_VIEW,
            rx: 0.0,
            ry: 0.0,
            rz: 0.0,
            dist: 0.0,
            height,
            angle: TILT,
            mode: CAMERA_MODE,
        };
        let transition = if initial { INSTANT } else { transition };
        // The native camera applies a zero-length transition on its next update.
        // Keep issuing the instant state for a few updates so the first rendered
        // frame cannot keep the default full-map camera.
        if spring::set_camera_state(camera, transition, 1.0, 1.0).is_ok_and(|success| success)
            && initial
        {
            self.initial_updates.set(self.initial_updates.get() + 1);
        }
    }
}

fn target(global: &Global) -> Option<(spring::types::Float3, spring::types::Float3, f32, f32)> {
    let scripted = global.scripted.access().camera;
    if let Some((x, z, height)) = scripted {
        let position = spring::types::Float3 { x, y: 0.0, z };
        return Some((position, spring::types::Float3::default(), INSTANT, height));
    }
    let unit_id = player_unit(global)?;
    let position = spring::units_info::get_unit_position(
        unit_id,
        spring::units_info::GetUnitPositionOptions {
            mid_pos: false,
            aim_pos: false,
        },
    )
    .ok()?;
    let velocity = spring::units_info::get_unit_velocity(unit_id).unwrap_or_default();
    Some((position, velocity, TRANSITION, HEIGHT))
}

const LEAD: f32 = 9.0;
const PITCH_Y: f32 = -0.9;
const PITCH_Z: f32 = -0.45;
const GRAVITY_WEAPON_RANGE: f32 = 420.0;
const HEIGHT: f32 = 2.0 * GRAVITY_WEAPON_RANGE;
const FIELD_OF_VIEW: f32 = 45.0;
const TILT: f32 = 0.464;
const CAMERA_MODE: i32 = 1;
const TRANSITION: f32 = 1.0;
const INSTANT: f32 = 0.0;
const INITIAL_CAMERA_UPDATES: u8 = 3;

fn player_unit(global: &Global) -> Option<i32> {
    if let Some(unit_id) = global.player_unit.get() {
        return Some(unit_id);
    }
    // Before the first status message: ask the engine.
    let player_id = spring::player::get_local_player_id(0).ok()?;
    let controlled = spring::teams::get_player_controlled_unit(player_id).ok()?;
    if !controlled.has_unit || controlled.unit_id < 0 {
        return None;
    }
    global.player_unit.set(Some(controlled.unit_id));
    Some(controlled.unit_id)
}

fn game_speed() -> f32 {
    spring::display::get_game_speed(0)
        .map(|speed| if speed.paused { 0.0 } else { speed.speed })
        .unwrap_or(1.0)
}
