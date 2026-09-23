use alloc::format;
use alloc::string::String;

use super::{Hud, model};

/// Show a GRAVITAS_STATUS message. Returns the player unit it reports.
pub fn update(hud: &mut Hud, message: &str) -> Option<i32> {
    if hud.status != message {
        hud.status = String::from(message);
    }
    show(hud, message)
}

/// Show a status message on the HUD.
pub fn show(hud: &Hud, message: &str) -> Option<i32> {
    let mut health = 0;
    let mut max_health = 1;
    let mut player_unit = None;
    let mut ready = [None, None];
    for field in message.split('|').skip(1) {
        let Some((key, value)) = field.split_once('=') else {
            continue;
        };
        match key {
            "health" => health = value.parse().unwrap_or(0),
            "maxHealth" => max_health = value.parse().unwrap_or(1),
            "playerUnit" => player_unit = value.parse().ok().filter(|unit_id| *unit_id >= 0),
            "push" => ready[0] = Some(value != "0"),
            "pull" => ready[1] = Some(value != "0"),
            _ => {}
        }
    }
    let Some(bindings) = hud.bindings else {
        return player_unit;
    };
    for (handle, ready) in [bindings.push_ready, bindings.pull_ready]
        .into_iter()
        .zip(ready)
    {
        if let Some(ready) = ready {
            model::set_bool(handle, ready);
        }
    }
    let max_health = max_health.max(1);
    let health = health.max(0);
    let ratio = (health as f32 / max_health as f32).clamp(0.0, 1.0);
    model::set_percent(bindings.health_percent, ratio * 100.0);
    model::set_string(bindings.health_caption, &format!("{health}/{max_health}"));
    model::set_string(bindings.health_color, &health_color(ratio));
    player_unit
}

fn health_color(ratio: f32) -> String {
    let [r, g, b] = crate::health::color(ratio).map(|channel| (255.0 * channel) as i32);
    format!("#{r:02x}{g:02x}{b:02x}")
}
