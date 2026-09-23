use alloc::string::String;

use spring::types::Float3;

use crate::player::PlayerCommand;
use crate::tests::arena::Arena;
use crate::tests::runner;
use crate::units;

pub trait Case {
    fn name(&self) -> &'static str;
    fn step(&mut self, frame: i32) -> Step;
}

pub enum Step {
    Wait,
    Pass,
    Fail(String),
}

pub fn fail(reason: &str) -> Step {
    Step::Fail(String::from(reason))
}

pub fn arena<R>(function: impl FnOnce(&Arena) -> R) -> Option<R> {
    runner::with_arena(function)
}

pub fn look(position: Float3, height: f32) {
    let _ = spring::send_to_unsynced(&alloc::format!(
        "GRAVITAS_SHOT|look|{}|{}|{height}",
        position.x,
        position.z
    ));
}

// Input is simulated on the synced side, because a headless run has no ui
// module. The PlayerCommand is the one the ui module's message would produce.
pub fn hold_jump(x: f32, z: f32) {
    // The height matters: a jump is refused as TooHigh unless the goal sits at
    // the terrain, which is what the ui module's cursor trace supplies.
    let command = PlayerCommand {
        target_x: x,
        target_y: units::ground_height(x, z),
        target_z: z,
        jump: true,
        ..PlayerCommand::default()
    };
    HELD.with(|held| held.set(Some(command)));
    player_command(command);
}

pub fn release_jump() {
    HELD.with(|held| held.set(None));
    player_command(PlayerCommand::default());
}

/// Re-issues a held command every frame, like the ui module's per-frame poll.
/// `Player::update` consumes `pending` each frame, so a single accept
/// would only ever be one frame of input.
pub fn pump_held_input() {
    let Some(command) = HELD.with(|held| held.get()) else {
        return;
    };
    player_command(command);
}

thread_local! {
    static HELD: core::cell::Cell<Option<PlayerCommand>> =
        const { core::cell::Cell::new(None) };
}

pub fn press_restart() {
    crate::restart::request();
}

fn player_command(command: PlayerCommand) {
    let _ = crate::state::with_world(|world| world.player.accept(command));
}

pub fn capture(name: &str) {
    let _ = spring::send_to_unsynced(&alloc::format!("GRAVITAS_SHOT|shot|{name}"));
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

pub fn heal(unit_id: i32, value: f32) {
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
}

pub fn place(unit_id: i32, position: Float3) {
    let _ = spring::set_unit_velocity(unit_id, Float3::default());
    let _ = spring::set_unit_position(unit_id, position);
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
    let _ = spring::give_order_to_unit(unit_id, crate::cmd::STOP, &[], 0, 0);
}
