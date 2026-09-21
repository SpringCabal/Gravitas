use crate::state;

#[cfg(feature = "e2e")]
pub fn look_at(x: f32, z: f32, height: Option<f32>) {
    let _ = state::with_state(|ui| ui.camera_override = Some((x, z, height.unwrap_or(HEIGHT))));
}

pub fn initialize() {
    // The first camera update must not wait for the first synced status packet.
    // The player-controlled-unit API is already available during UI startup,
    // so use it here and let follow() keep the status-packet fallback below.
    follow();
}

pub fn follow() {
    let Some((position, velocity, transition, height)) = target() else {
        return;
    };
    let initial = state::with_state(|ui| !ui.camera_initialized).unwrap_or(false);
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
    if spring::set_camera_state(camera, transition, 1.0, 1.0).is_ok_and(|success| success) {
        let _ = state::with_state(|ui| {
            if initial {
                // The native camera applies a zero-length transition on its
                // next update. Keep issuing the instant state for a few
                // update callbacks so the first rendered frame cannot retain
                // the default full-map camera.
                ui.camera_initial_updates = ui.camera_initial_updates.saturating_add(1);
                if ui.camera_initial_updates >= INITIAL_CAMERA_UPDATES {
                    ui.camera_initialized = true;
                }
            }
        });
    }
}

fn target() -> Option<(spring::types::Float3, spring::types::Float3, f32, f32)> {
    if let Some((x, z, height)) = state::with_state(|ui| ui.camera_override).flatten() {
        let position = spring::types::Float3 { x, y: 0.0, z };
        return Some((position, spring::types::Float3::default(), INSTANT, height));
    }
    let unit_id = player_unit()?;
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

fn player_unit() -> Option<i32> {
    let stored = state::with_state(|ui| ui.player_unit).unwrap_or(-1);
    if stored >= 0 {
        return Some(stored);
    }

    let player_id = spring::player::get_local_player_id(0).ok()?;
    let controlled = spring::teams::get_player_controlled_unit(player_id).ok()?;
    if !controlled.has_unit || controlled.unit_id < 0 {
        return None;
    }
    let unit_id = controlled.unit_id;
    let _ = state::with_state(|ui| ui.player_unit = unit_id);
    Some(unit_id)
}

fn game_speed() -> f32 {
    spring::display::get_game_speed(0)
        .map(|speed| if speed.paused { 0.0 } else { speed.speed })
        .unwrap_or(1.0)
}
