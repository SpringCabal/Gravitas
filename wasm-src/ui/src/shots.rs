#[cfg(feature = "e2e")]
pub fn handle(message: &str) {
    let mut fields = message.split('|').skip(1);
    match fields.next() {
        Some("shot") => {
            let _ = spring::send_commands("screenshot", "");
        }
        Some("look") => {
            look(&mut fields);
        }
        Some("hold") => hold(&mut fields),
        Some("restart") => {
            let _ = spring::send_lua_rules_msg("gravitas_restart");
        }
        Some("release") => release(),
        Some("done") => {
            let _ = spring::send_commands("quitforce", "");
        }
        _ => {}
    }
}

#[cfg(not(feature = "e2e"))]
pub fn handle(_message: &str) {}

#[cfg(feature = "e2e")]
fn hold<'a>(fields: &mut impl Iterator<Item = &'a str>) {
    let (Some(x), Some(z)) = (number(fields), number(fields)) else {
        return;
    };
    let _ = crate::state::with_state(|ui| {
        ui.jump = true;
        ui.scripted_target = Some((x, z));
    });
}

#[cfg(feature = "e2e")]
fn release() {
    let _ = crate::state::with_state(|ui| {
        ui.jump = false;
        ui.scripted_target = None;
    });
}

#[cfg(feature = "e2e")]
fn look<'a>(fields: &mut impl Iterator<Item = &'a str>) {
    let (Some(x), Some(z)) = (number(fields), number(fields)) else {
        return;
    };
    crate::camera::look_at(x, z, number(fields));
}

#[cfg(feature = "e2e")]
fn number<'a>(fields: &mut impl Iterator<Item = &'a str>) -> Option<f32> {
    fields
        .next()
        .and_then(|value| value.parse::<f32>().ok())
        .filter(|value| value.is_finite())
}
