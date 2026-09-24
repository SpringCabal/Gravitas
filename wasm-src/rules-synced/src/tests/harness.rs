use alloc::string::String;

use spring::types::Float3;

use crate::player::PlayerCommand;
use crate::state::Global;
use crate::tests::arena::Arena;
use crate::units;

pub trait Case {
    fn name(&self) -> &'static str;
    fn step(&mut self, global: &Global, frame: i32) -> Step;
}

pub enum Step {
    Wait,
    Pass,
    Fail(String),
}

pub fn fail(reason: &str) -> Step {
    Step::Fail(String::from(reason))
}

pub fn arena<R>(global: &Global, function: impl FnOnce(&Arena) -> R) -> Option<R> {
    global.tests.arena.get().map(|arena| function(&arena))
}

pub fn look(position: Float3, height: f32) {
    let _ = spring::send_to_unsynced(&alloc::format!(
        "GRAVITAS_SHOT|look|{}|{}|{height}",
        position.x,
        position.z
    ));
}

// Input is simulated on the synced side, because a headless run has no ui
// module. The PlayerCommand is the one the ui module's message would produce:
// sent when it changes, with `jump` set on the message where the key went down.
pub fn press_jump(global: &Global, x: f32, z: f32) {
    // The height matters: a jump is refused as TooHigh unless the goal sits at
    // the terrain, which is what the ui module's cursor trace supplies.
    let command = PlayerCommand {
        target_x: x,
        target_y: units::ground_height(x, z),
        target_z: z,
        jump: true,
        ..PlayerCommand::default()
    };
    player_command(global, command);
}

pub fn release_jump(global: &Global) {
    player_command(global, PlayerCommand::default());
}

pub fn press_restart(global: &Global) {
    global.restart.request();
}

fn player_command(global: &Global, command: PlayerCommand) {
    crate::player::accept(&global.player_input, command);
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
    let _ = spring::give_order_to_unit(unit_id, spring::cmd::STOP, &[], 0, 0);
}
