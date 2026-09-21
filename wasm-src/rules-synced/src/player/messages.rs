use super::control::PlayerCommand;

pub fn parse(data: &[u8]) -> Option<PlayerCommand> {
    let message = core::str::from_utf8(data).ok()?;
    let mut fields = message.split('|');
    if fields.next()? != "gravitas" {
        return None;
    }
    Some(PlayerCommand {
        move_x: number(&mut fields)?,
        move_z: number(&mut fields)?,
        target_x: number(&mut fields)?,
        target_y: number(&mut fields)?,
        target_z: number(&mut fields)?,
        target_unit: number(&mut fields)? as i32,
        push: flag(&mut fields)?,
        pull: flag(&mut fields)?,
        jump: flag(&mut fields)?,
    })
}

fn number<'a>(fields: &mut impl Iterator<Item = &'a str>) -> Option<f32> {
    fields
        .next()
        .and_then(|value| value.parse::<f32>().ok())
        .filter(|value| value.is_finite())
}

fn flag<'a>(fields: &mut impl Iterator<Item = &'a str>) -> Option<bool> {
    fields.next().map(|value| value == "1")
}
