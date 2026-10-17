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

pub fn create(spring: &spring::Spring<'_, spring::Ui>, context: u64) -> Option<HudBindings> {
    let model = spring
        .rml_ui()
        .context_create_data_model(context, "gravitas")
        .ok()?;
    if !model.success || model.data_model_handle == 0 {
        return None;
    }
    let model = model.data_model_handle;
    bind_event(spring, model, "endgameRestart", restart)?;
    bind_event(spring, model, "endgameExit", |spring, _| {
        springcabal::menu::quit_to_menu(spring)
    })?;
    Some(HudBindings {
        health_percent: bind_percent(spring, model, "healthPercent", 100.0)?,
        health_caption: bind_string(spring, model, "healthCaption", "")?,
        health_color: bind_string(spring, model, "healthColor", HEALTH_FULL_COLOR)?,
        push_ready: bind_bool(spring, model, "pushReady", true)?,
        pull_ready: bind_bool(spring, model, "pullReady", true)?,
        paused: bind_bool(spring, model, "paused", false)?,
        endgame_visible: bind_bool(spring, model, "endgameVisible", false)?,
        endgame_victory: bind_bool(spring, model, "endgameVictory", false)?,
        endgame_title: bind_string(spring, model, "endgameTitle", "")?,
    })
}

pub fn set_percent(spring: &spring::Spring<'_, spring::Ui>, handle: u64, value: f32) {
    if handle != 0 {
        let _ = spring
            .rml_ui()
            .data_model_set_percent(handle, value.clamp(0.0, 100.0));
    }
}

pub fn set_bool(spring: &spring::Spring<'_, spring::Ui>, handle: u64, value: bool) {
    if handle != 0 {
        let _ = spring.rml_ui().data_model_set_bool(handle, value);
    }
}

pub fn set_string(spring: &spring::Spring<'_, spring::Ui>, handle: u64, value: &str) {
    if handle != 0 {
        let _ = spring.rml_ui().data_model_set_string(handle, value);
    }
}

fn bind_event(
    spring: &spring::Spring<'_, spring::Ui>,
    model: u64,
    name: &str,
    handler: impl Fn(&spring::Spring<'_, spring::Ui>, &Global) + spring::RetainedCapture + 'static,
) -> Option<()> {
    spring
        .rml_ui()
        .data_model_bind_event(model, name, &[], move |spring, _| {
            crate::with_global(|global| handler(spring, global));
            Ok(())
        })
        .is_ok_and(|registration| registration.success)
        .then_some(())
}

fn bind_percent(
    spring: &spring::Spring<'_, spring::Ui>,
    model: u64,
    name: &str,
    initial: f32,
) -> Option<u64> {
    let binding = spring
        .rml_ui()
        .data_model_bind_percent(model, name, initial)
        .ok()?;
    binding.success.then_some(binding.variable_handle)
}

fn bind_bool(
    spring: &spring::Spring<'_, spring::Ui>,
    model: u64,
    name: &str,
    initial: bool,
) -> Option<u64> {
    let binding = spring
        .rml_ui()
        .data_model_bind_bool(model, name, initial)
        .ok()?;
    binding.success.then_some(binding.variable_handle)
}

fn bind_string(
    spring: &spring::Spring<'_, spring::Ui>,
    model: u64,
    name: &str,
    initial: &str,
) -> Option<u64> {
    let binding = spring
        .rml_ui()
        .data_model_bind_string(model, name, initial)
        .ok()?;
    binding.success.then_some(binding.variable_handle)
}

pub fn restart(spring: &spring::Spring<'_, spring::Ui>, global: &Global) {
    springcabal::message::send_to_rules(spring, gravitas_shared::RESTART);
    super::overlays::hide_game_over(spring, &mut global.hud.access_mut());
}
