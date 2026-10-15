use core::cell::Cell;

use spring::types::Float3;
use springcabal_control::camera::{Aim, CameraFollow, FocusY, Follow, View, place_camera};

use crate::state::Global;

const GRAVITY_WEAPON_RANGE: f32 = 420.0;
const HEIGHT: f32 = 2.0 * GRAVITY_WEAPON_RANGE;
const TILT: f32 = 0.464;
const VIEW: View = View::overhead(HEIGHT, TILT).pitch(-0.9, -0.45).fov(45.0);
const LEAD_FRAMES: f32 = 9.0;
const TRANSITION: f32 = 1.0;
const SNAP_UPDATES: u8 = 3;
const FOLLOW: Follow = Follow::state(VIEW, TRANSITION)
    .aim(Aim::Position)
    .lead(LEAD_FRAMES)
    .focus_y(FocusY::Fixed(0.0))
    .snap_updates(SNAP_UPDATES);

pub struct Camera {
    follow: CameraFollow,
    scripted: Cell<Option<(f32, f32, f32)>>,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            follow: CameraFollow::new(FOLLOW),
            scripted: Cell::new(None),
        }
    }
}

impl Camera {
    #[cfg(feature = "headless-integration")]
    pub fn handle_request(&self, message: &[u8]) {
        use gravitas_shared::e2e::Request;
        if let Some(Request::Look { x, z, height }) =
            core::str::from_utf8(message).ok().and_then(Request::parse)
        {
            self.scripted.set(Some((x, z, height.unwrap_or(HEIGHT))));
        }
    }

    pub fn follow(&self, global: &Global, dt: f32) {
        if let Some((x, z, height)) = self.scripted.get() {
            place_camera(&VIEW.height(height), Float3 { x, y: 0.0, z });
            return;
        }
        self.follow.update(player_unit(global), dt);
    }
}

#[cfg(feature = "fire-probe")]
pub fn look_at(x: f32, z: f32, height: Option<f32>) {
    place_camera(
        &VIEW.height(height.unwrap_or(HEIGHT)),
        Float3 { x, y: 0.0, z },
    );
}

fn player_unit(global: &Global) -> Option<i32> {
    if let Some(unit_id) = global.player_unit.get() {
        return Some(unit_id);
    }
    let player_id = spring::player::get_local_player_id(0).ok()?;
    let controlled = spring::teams::get_player_controlled_unit(player_id).ok()?;
    if !controlled.has_unit || controlled.unit_id < 0 {
        return None;
    }
    global.player_unit.set(Some(controlled.unit_id));
    Some(controlled.unit_id)
}
