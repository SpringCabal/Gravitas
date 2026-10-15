use super::control::PlayerCommand;

pub fn parse(data: &[u8]) -> Option<PlayerCommand> {
    let input = gravitas_shared::Input::parse(core::str::from_utf8(data).ok()?)?;
    let [target_x, target_y, target_z] = input.target;
    Some(PlayerCommand {
        move_x: input.move_x,
        move_z: input.move_z,
        target_x,
        target_y,
        target_z,
        target_unit: input.target_unit,
        push: input.push,
        pull: input.pull,
        jump: input.jump,
    })
}
