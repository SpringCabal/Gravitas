use super::{Hud, model};

pub fn update_pause(spring: &spring::Spring<'_, spring::Ui>, hud: &mut Hud) {
    let paused = spring
        .display()
        .get_game_speed()
        .map(|speed| speed.paused)
        .unwrap_or(false);
    if hud.was_paused == paused {
        return;
    }
    hud.was_paused = paused;
    if let Some(bindings) = hud.bindings {
        model::set_bool(spring, bindings.paused, paused);
    }
}

pub fn hide_game_over(spring: &spring::Spring<'_, spring::Ui>, hud: &mut Hud) {
    hud.endgame = None;
    if let Some(bindings) = hud.bindings {
        model::set_bool(spring, bindings.endgame_visible, false);
    }
}

pub fn game_over(spring: &spring::Spring<'_, spring::Ui>, hud: &mut Hud, victory: bool) {
    if hud.endgame.is_some() {
        return;
    }
    hud.endgame = Some(victory);
    show(spring, hud);
}

pub fn show(spring: &spring::Spring<'_, spring::Ui>, hud: &Hud) {
    let Some(bindings) = hud.bindings else {
        return;
    };
    model::set_bool(spring, bindings.paused, hud.was_paused);
    model::set_bool(spring, bindings.endgame_visible, hud.endgame.is_some());
    let Some(victory) = hud.endgame else {
        return;
    };
    let title = if victory { "You win!" } else { "You died." };
    model::set_bool(spring, bindings.endgame_victory, victory);
    model::set_string(spring, bindings.endgame_title, title);
}
