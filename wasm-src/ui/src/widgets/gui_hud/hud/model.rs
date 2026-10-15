use crate::state::Global;

#[derive(Clone, Copy, Default)]
pub struct HudBindings {
    pub health_percent: u64,
    pub health_caption: u64,
    pub health_color: u64,
    pub push_ready: u64,
    pub pull_ready: u64,
    pub paused: u64,
    pub endgame_visible: u64,
    pub endgame_victory: u64,
    pub endgame_title: u64,
}

pub const HEALTH_FULL_COLOR: &str = "#00bf00";

pub fn create(context: u64) -> Option<HudBindings> {
    let model = spring::context_create_data_model(context, "gravitas").ok()?;
    if !model.success || model.data_model_handle == 0 {
        return None;
    }
    let model = model.data_model_handle;
    bind_event(model, "endgameRestart", restart)?;
    bind_event(model, "endgameExit", |_| springcabal::menu::quit_to_menu())?;
    Some(HudBindings {
        health_percent: bind_percent(model, "healthPercent", 100.0)?,
        health_caption: bind_string(model, "healthCaption", "")?,
        health_color: bind_string(model, "healthColor", HEALTH_FULL_COLOR)?,
        push_ready: bind_bool(model, "pushReady", true)?,
        pull_ready: bind_bool(model, "pullReady", true)?,
        paused: bind_bool(model, "paused", false)?,
        endgame_visible: bind_bool(model, "endgameVisible", false)?,
        endgame_victory: bind_bool(model, "endgameVictory", false)?,
        endgame_title: bind_string(model, "endgameTitle", "")?,
    })
}

pub fn set_percent(handle: u64, value: f32) {
    if handle != 0 {
        let _ = spring::data_model_set_percent(handle, value.clamp(0.0, 100.0));
    }
}

pub fn set_bool(handle: u64, value: bool) {
    if handle != 0 {
        let _ = spring::data_model_set_bool(handle, value);
    }
}

pub fn set_string(handle: u64, value: &str) {
    if handle != 0 {
        let _ = spring::data_model_set_string(handle, value);
    }
}

fn bind_event(model: u64, name: &str, handler: impl FnMut(&Global) + 'static) -> Option<()> {
    let callback = crate::register_callback(handler);
    let bound = spring::data_model_bind_event(model, name, callback, &[])
        .is_ok_and(|registration| registration.success);
    if !bound {
        crate::unregister_callback(callback);
    }
    bound.then_some(())
}

fn bind_percent(model: u64, name: &str, initial: f32) -> Option<u64> {
    let binding = spring::data_model_bind_percent(model, name, initial).ok()?;
    binding.success.then_some(binding.variable_handle)
}

fn bind_bool(model: u64, name: &str, initial: bool) -> Option<u64> {
    let binding = spring::data_model_bind_bool(model, name, initial).ok()?;
    binding.success.then_some(binding.variable_handle)
}

fn bind_string(model: u64, name: &str, initial: &str) -> Option<u64> {
    let binding = spring::data_model_bind_string(model, name, initial).ok()?;
    binding.success.then_some(binding.variable_handle)
}

pub fn restart(global: &Global) {
    springcabal::message::send_to_rules(gravitas_shared::RESTART);
    super::overlays::hide_game_over(&mut global.hud.access_mut());
}
