use alloc::format;
use alloc::string::String;

use crate::state;

use super::model;

pub fn update(message: &str) {
    let mut health = 0;
    let mut max_health = 1;
    for field in message.split('|').skip(1) {
        let Some((key, value)) = field.split_once('=') else {
            continue;
        };
        match key {
            "health" => health = value.parse().unwrap_or(0),
            "maxHealth" => max_health = value.parse().unwrap_or(1),
            "frame" => {
                let frame = value.parse().unwrap_or(0);
                let _ = state::with_state(|ui| ui.game_frame = frame);
            }
            "playerUnit" => {
                let unit_id = value.parse().unwrap_or(-1);
                let _ = state::with_state(|ui| ui.player_unit = unit_id);
            }
            "push" => set_ready(value != "0", true),
            "pull" => set_ready(value != "0", false),
            _ => {}
        }
    }
    apply_health(health, max_health);
}

fn set_ready(ready: bool, is_push: bool) {
    let _ = state::with_state(|ui| {
        let Some(hud) = ui.hud else {
            return;
        };
        if is_push {
            model::set_bool(hud.push_ready, ready);
        } else {
            model::set_bool(hud.pull_ready, ready);
        }
    });
}

fn apply_health(health: i32, max_health: i32) {
    let max_health = max_health.max(1);
    let health = health.max(0);
    let ratio = (health as f32 / max_health as f32).clamp(0.0, 1.0);
    let caption = format!("{health}/{max_health}");
    let color = health_color(ratio);
    let _ = state::with_state(|ui| {
        let Some(hud) = ui.hud else {
            return;
        };
        model::set_percent(hud.health_percent, ratio * 100.0);
        model::set_string(hud.health_caption, &caption);
        model::set_string(hud.health_color, &color);
    });
}

fn health_color(ratio: f32) -> String {
    let [r, g, b] = crate::health::color(ratio).map(|channel| (255.0 * channel) as i32);
    format!("#{r:02x}{g:02x}{b:02x}")
}
