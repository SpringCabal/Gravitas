use crate::state;

use super::model;

pub fn update_pause() {
    let paused = spring::display::get_game_speed(0)
        .map(|speed| speed.paused)
        .unwrap_or(false);
    let _ = state::with_state(|ui| {
        if ui.was_paused == paused {
            return;
        }
        ui.was_paused = paused;
        if let Some(hud) = ui.hud {
            model::set_bool(hud.paused, paused);
        }
    });
}

pub fn hide_game_over() {
    let _ = state::with_state(|ui| {
        ui.endgame_open = false;
        let Some(hud) = ui.hud else {
            return;
        };
        model::set_bool(hud.endgame_visible, false);
    });
}

pub fn game_over(victory: bool) {
    let title = if victory { "You win!" } else { "You died." };
    let _ = state::with_state(|ui| {
        if ui.endgame_open {
            return;
        }
        ui.endgame_open = true;
        let Some(hud) = ui.hud else {
            return;
        };
        model::set_bool(hud.endgame_visible, true);
        model::set_bool(hud.endgame_victory, victory);
        model::set_string(hud.endgame_title, title);
    });
}
