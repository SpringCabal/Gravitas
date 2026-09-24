use super::control::PlayerCommand;

/// `gravitas|move_x|move_z|target_x|target_y|target_z|target_unit|push|pull|jump`,
/// sent by the ui module's input widget.
pub fn parse(data: &[u8]) -> Option<PlayerCommand> {
    let message = core::str::from_utf8(data).ok()?;
    let mut fields = springcabal::message::fields(message, "gravitas")?;
    Some(PlayerCommand {
        move_x: fields.f32()?,
        move_z: fields.f32()?,
        target_x: fields.f32()?,
        target_y: fields.f32()?,
        target_z: fields.f32()?,
        target_unit: fields.i32()?,
        push: fields.flag()?,
        pull: fields.flag()?,
        jump: fields.flag()?,
    })
}
