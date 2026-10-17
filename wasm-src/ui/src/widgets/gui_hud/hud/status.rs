use alloc::format;
use alloc::string::String;

use gravitas_shared::Status;

use super::{Hud, model};

pub fn update(spring: &spring::Spring<'_, spring::Ui>, hud: &mut Hud, status: Status) {
    hud.status = Some(status);
    show(spring, hud, status);
}

pub fn show(spring: &spring::Spring<'_, spring::Ui>, hud: &Hud, status: Status) {
    let Some(bindings) = hud.bindings else {
        return;
    };
    model::set_bool(spring, bindings.push_ready, status.push_ready);
    model::set_bool(spring, bindings.pull_ready, status.pull_ready);
    let max_health = status.max_health.max(1);
    let health = status.health.max(0);
    let ratio = (health as f32 / max_health as f32).clamp(0.0, 1.0);
    model::set_percent(spring, bindings.health_percent, ratio * 100.0);
    model::set_string(
        spring,
        bindings.health_caption,
        &format!("{health}/{max_health}"),
    );
    model::set_string(spring, bindings.health_color, &health_color(ratio));
}

fn health_color(ratio: f32) -> String {
    let [r, g, b] = crate::health::color(ratio).map(|channel| (255.0 * channel) as i32);
    format!("#{r:02x}{g:02x}{b:02x}")
}
