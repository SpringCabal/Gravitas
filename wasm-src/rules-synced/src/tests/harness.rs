use alloc::string::String;
use alloc::vec::Vec;

use gravitas_shared::e2e::Request;
use spring::commands;
use spring::types::Float3;

use crate::state::Global;
use crate::tests::arena::Arena;
use crate::units;

pub trait Case {
    fn name(&self) -> &'static str;
    fn phase(&self) -> Phase {
        Phase::Arena
    }
    /// `frame` counts from the case's start: 0 on its first step.
    fn step(&mut self, global: &Global, frame: i32) -> Step;
    /// Where the case left its units once it passed. The scene case checks
    /// them on the frame it has the arena shot.
    fn rest(&self) -> Vec<Rest> {
        Vec::new()
    }
}

/// When a case may start: every case of a phase starts together, once every
/// case of the phase before has its verdict. A case that changes more than
/// its own rig (the scene, a restart) gets a phase of its own.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Phase {
    /// The arena as built, before any case touches it.
    Setup,
    /// The cases that each play in their own rig.
    Arena,
    /// The arena at rest, shot and checked.
    Scene,
    /// Restarts rebuild the arena.
    Restart,
    /// Restarts into the real level.
    Level,
}

pub const PHASES: [Phase; 5] = [
    Phase::Setup,
    Phase::Arena,
    Phase::Scene,
    Phase::Restart,
    Phase::Level,
];

/// Where a unit should be once its case is done.
pub enum Rest {
    /// Within `within` elmos of `at`, on the ground plane.
    At {
        unit: i32,
        at: Float3,
        within: f32,
        what: &'static str,
    },
    Gone {
        unit: i32,
        what: &'static str,
    },
}

impl Rest {
    pub fn at(unit: i32, at: Float3, within: f32, what: &'static str) -> Self {
        Rest::At {
            unit,
            at,
            within,
            what,
        }
    }

    /// Where `unit` is now, with `within` of leeway.
    pub fn here(unit: i32, within: f32, what: &'static str) -> Option<Self> {
        position(unit).map(|at| Rest::at(unit, at, within, what))
    }
}

pub enum Step {
    Wait,
    Pass,
    Fail(String),
}

/// Leeway for a unit no case moved: it settles on the ground after spawning.
pub const STILL: f32 = 10.0;

pub fn fail(reason: &str) -> Step {
    Step::Fail(String::from(reason))
}

pub fn arena<R>(global: &Global, function: impl FnOnce(&Arena) -> R) -> Option<R> {
    global.tests.arena.get().map(|arena| function(&arena))
}

pub fn look(position: Float3, height: f32) {
    send_to_ui(Request::Look {
        x: position.x,
        z: position.z,
        height: Some(height),
    });
}

// Input goes through the ui module, which the headless run also loads: it
// points its input widget where asked and sends the usual input message, and
// it presses the HUD's restart button.

/// Point at the ground at `x`, `z` and press jump (the middle button).
pub fn press_jump(x: f32, z: f32) {
    // The height matters: a jump is refused as TooHigh unless the goal sits at
    // the terrain, which is what the ui module's cursor trace supplies.
    let y = units::ground_height(x, z);
    send_to_ui(Request::Pointer {
        target: [x, y, z],
        jump: true,
    });
}

/// Give the mouse back to the ui module.
pub fn release_pointer() {
    send_to_ui(Request::PointerOff);
}

pub fn press_restart() {
    send_to_ui(Request::Restart);
}

pub fn send_to_ui(request: Request<'_>) {
    springcabal::message::send_to_ui(&request.encode());
}

pub fn capture(name: &str) {
    send_to_ui(Request::Shot(name));
}

pub fn health(unit_id: i32) -> Option<f32> {
    spring::units_info::get_unit_health(unit_id)
        .ok()
        .map(|health| health.health)
}

pub fn paralyze(unit_id: i32) -> Option<f32> {
    spring::units_info::get_unit_health(unit_id)
        .ok()
        .map(|health| health.paralyze_damage)
}

/// Set the unit's health and clear its stun, once this callin returns.
pub fn heal(unit_id: i32, value: f32) {
    commands::run(move || {
        let Ok(health) = spring::units_info::get_unit_health(unit_id) else {
            return;
        };
        let _ = spring::set_unit_health(
            unit_id,
            spring::types::UnitHealthValue {
                health: value,
                capture: health.capture_progress,
                paralyze: 0.0,
                build: health.build_progress,
                use_amounts: true,
            },
        );
    });
}

/// Move the unit there, stopped, once this callin returns: after the orders
/// given before.
pub fn place(unit_id: i32, position: Float3) {
    commands::run(move || {
        let _ = spring::set_unit_velocity(unit_id, Float3::default());
        let _ = spring::set_unit_position(unit_id, position);
    });
}

pub fn position(unit_id: i32) -> Option<Float3> {
    units::position(unit_id)
}

pub fn piece_position(unit_id: i32, name: &str) -> Option<Float3> {
    let names = spring::units_pieces::get_unit_piece_list(unit_id).ok()?;
    let index = names.iter().position(|candidate| candidate == name)?;
    spring::units_pieces::get_unit_piece_position(unit_id, index as i32 + 1).ok()
}

pub fn distance(from: Float3, to: Float3) -> f32 {
    let dx = to.x - from.x;
    let dy = to.y - from.y;
    let dz = to.z - from.z;
    (dx * dx + dy * dy + dz * dz).sqrt()
}

pub fn planar_distance(from: Float3, to: Float3) -> f32 {
    let dx = to.x - from.x;
    let dz = to.z - from.z;
    (dx * dx + dz * dz).sqrt()
}

pub fn moved(from: Option<Float3>, to: Option<Float3>, minimum: f32) -> bool {
    let (Some(from), Some(to)) = (from, to) else {
        return false;
    };
    distance(from, to) > minimum
}

pub fn halt(unit_id: i32) {
    let _ = commands::give_order_to_unit(unit_id, spring::cmd::STOP, &[], 0, 0);
}

/// Destroy the unit once this callin returns.
pub fn destroy(unit_id: i32) {
    let _ = commands::destroy_unit(
        unit_id,
        spring::generated::owned::unit_control::DestroyUnitOptions {
            selfd: false,
            reclaimed: false,
            attacker_id: -1,
            recycle_id: false,
        },
    );
}
