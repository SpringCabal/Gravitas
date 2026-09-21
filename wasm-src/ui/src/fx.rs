pub fn handle(message: &str) {
    let mut fields = message.split('|').skip(1);
    let Some(kind) = fields.next() else {
        return;
    };
    let Some(x) = number(&mut fields) else {
        return;
    };
    let Some(y) = number(&mut fields) else {
        return;
    };
    let Some(z) = number(&mut fields) else {
        return;
    };
    let magnitude = number(&mut fields).unwrap_or(1.0);
    if !in_view(x, y, z, magnitude) {
        return;
    }
    let position = spring::types::Float3 { x, y, z };
    match kind {
        "burn" => {
            let size_mult = number(&mut fields).unwrap_or(1.0);
            let velocity_x = number(&mut fields).unwrap_or(0.0);
            let velocity_z = number(&mut fields).unwrap_or(0.0);
            crate::fire::burn(position, magnitude, size_mult, [velocity_x, velocity_z]);
        }
        "flame" => crate::fire::flame(position, magnitude),
        "arc" => crate::fire::electric_field(position, magnitude),
        "spark" => {
            let velocity_x = number(&mut fields).unwrap_or(0.0);
            let velocity_z = number(&mut fields).unwrap_or(0.0);
            crate::fire::electric_spark(position, magnitude, [velocity_x, velocity_z]);
        }
        _ => {}
    }
}

fn in_view(x: f32, y: f32, z: f32, magnitude: f32) -> bool {
    let centre = spring::types::Float3 { x, y, z };
    spring::display::is_sphere_in_view(centre, magnitude.max(VIEW_RADIUS)).unwrap_or(true)
}

const VIEW_RADIUS: f32 = 150.0;

fn number<'a>(fields: &mut impl Iterator<Item = &'a str>) -> Option<f32> {
    fields
        .next()
        .and_then(|value| value.parse::<f32>().ok())
        .filter(|value| value.is_finite())
}
