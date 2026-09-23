use super::{Hud, model};

pub fn update_pause(hud: &mut Hud) {
    let paused = spring::display::get_game_speed(0)
        .map(|speed| speed.paused)
        .unwrap_or(false);
    if hud.was_paused == paused {
        return;
    }
    hud.was_paused = paused;
    if let Some(bindings) = hud.bindings {
        model::set_bool(bindings.paused, paused);
    }
}

pub fn hide_game_over(hud: &mut Hud) {
    hud.endgame_open = false;
    if let Some(bindings) = hud.bindings {
        model::set_bool(bindings.endgame_visible, false);
    }
}

pub fn game_over(hud: &mut Hud, victory: bool) {
    if hud.endgame_open {
        return;
    }
    hud.endgame_open = true;
    let Some(bindings) = hud.bindings else {
        return;
    };
    let title = if victory { "You win!" } else { "You died." };
    model::set_bool(bindings.endgame_visible, true);
    model::set_bool(bindings.endgame_victory, victory);
    model::set_string(bindings.endgame_title, title);
}
