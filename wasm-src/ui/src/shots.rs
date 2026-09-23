#[cfg(feature = "e2e")]
pub fn handle(global: &crate::state::Global, message: &str) {
    let mut fields = message.split('|').skip(1);
    match fields.next() {
        Some("shot") => {
            let _ = spring::send_commands("screenshot", "");
        }
        Some("look") => {
            look(global, &mut fields);
        }
        Some("hold") => hold(global, &mut fields),
        Some("restart") => {
            let _ = spring::send_lua_rules_msg("gravitas_restart");
        }
        Some("release") => release(global),
        Some("done") => {
            let _ = spring::send_commands("quitforce", "");
        }
        _ => {}
    }
}

#[cfg(not(feature = "e2e"))]
pub fn handle(_global: &crate::state::Global, _message: &str) {}

#[cfg(feature = "e2e")]
fn hold<'a>(global: &crate::state::Global, fields: &mut impl Iterator<Item = &'a str>) {
    let (Some(x), Some(z)) = (number(fields), number(fields)) else {
        return;
    };
    let mut scripted = global.scripted.access_mut();
    scripted.jump = true;
    scripted.target = Some((x, z));
}

#[cfg(feature = "e2e")]
fn release(global: &crate::state::Global) {
    let mut scripted = global.scripted.access_mut();
    scripted.jump = false;
    scripted.target = None;
}

#[cfg(feature = "e2e")]
fn look<'a>(global: &crate::state::Global, fields: &mut impl Iterator<Item = &'a str>) {
    let (Some(x), Some(z)) = (number(fields), number(fields)) else {
        return;
    };
    crate::camera::look_at(global, x, z, number(fields));
}

#[cfg(feature = "e2e")]
fn number<'a>(fields: &mut impl Iterator<Item = &'a str>) -> Option<f32> {
    fields
        .next()
        .and_then(|value| value.parse::<f32>().ok())
        .filter(|value| value.is_finite())
}
