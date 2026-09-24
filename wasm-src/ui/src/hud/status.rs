use alloc::format;
use alloc::string::String;

use gravitas_shared::Status;

use super::{Hud, model};

/// Show a status; it is shown again once the HUD exists.
pub fn update(hud: &mut Hud, status: Status) {
    hud.status = Some(status);
    show(hud, status);
}

/// Show a status on the HUD.
pub fn show(hud: &Hud, status: Status) {
    let Some(bindings) = hud.bindings else {
        return;
    };
    model::set_bool(bindings.push_ready, status.push_ready);
    model::set_bool(bindings.pull_ready, status.pull_ready);
    let max_health = status.max_health.max(1);
    let health = status.health.max(0);
    let ratio = (health as f32 / max_health as f32).clamp(0.0, 1.0);
    model::set_percent(bindings.health_percent, ratio * 100.0);
    model::set_string(bindings.health_caption, &format!("{health}/{max_health}"));
    model::set_string(bindings.health_color, &health_color(ratio));
}

fn health_color(ratio: f32) -> String {
    let [r, g, b] = crate::health::color(ratio).map(|channel| (255.0 * channel) as i32);
    format!("#{r:02x}{g:02x}{b:02x}")
}
