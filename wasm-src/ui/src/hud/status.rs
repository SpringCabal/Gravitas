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

const HEALTH_RAMP: [(f32, f32, f32); 3] = [(1.0, 0.0, 0.0), (0.8, 0.6, 0.0), (0.0, 0.75, 0.0)];

fn health_color(ratio: f32) -> String {
    let position = (HEALTH_RAMP.len() - 1) as f32 * ratio;
    let index = (position as usize).min(HEALTH_RAMP.len() - 2);
    let blend = position - index as f32;
    let low = HEALTH_RAMP[index];
    let high = HEALTH_RAMP[index + 1];
    let channel = |low: f32, high: f32| (255.0 * (low * (1.0 - blend) + high * blend)) as i32;
    format!(
        "#{:02x}{:02x}{:02x}",
        channel(low.0, high.0),
        channel(low.1, high.1),
        channel(low.2, high.2)
    )
}
