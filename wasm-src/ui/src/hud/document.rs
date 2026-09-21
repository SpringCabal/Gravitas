use alloc::format;

use crate::diagnostics;
use crate::state;

use super::model;

const FOCUS_NONE: i32 = 0;

pub fn initialize() {
    if state::with_state(|ui| ui.context != 0).unwrap_or(true) {
        return;
    }
    match spring::is_ready(0) {
        Ok(true) => {}
        Ok(false) => return,
        Err(error) => {
            diagnostics::warning(&format!(
                "RmlUi readiness query failed (error code {})",
                error.code
            ));
            return;
        }
    }
    let Some(context) = create_context() else {
        return;
    };
    let Some(hud) = model::create(context) else {
        diagnostics::warning("RmlUi data-model creation failed");
        return;
    };
    let Some(document) = load_document(context) else {
        return;
    };
    let _ = state::with_state(|ui| {
        ui.context = context;
        ui.document = document;
        ui.hud = Some(hud);
    });
    diagnostics::info("Gravitas RmlUi HUD initialized");
}

pub fn hide_default_layout() {
    if state::with_state(|ui| ui.default_layout_hidden).unwrap_or(true) {
        return;
    }
    let commands = [
        "Console 0",
        "ResBar 0",
        "ToolTip 0",
        "Clock 0",
        "Info 0",
        "showhealthbars 0",
        "showrezbars 0",
        "disticon 99999",
        "endgraph 0",
    ];
    let commands_hidden = commands
        .into_iter()
        .all(|command| spring::send_commands(command, "").is_ok());
    let minimap_hidden =
        spring::slave_mini_map(true).is_ok() && spring::config_mini_map(-1, -1, -1, -1).is_ok();
    if commands_hidden && minimap_hidden {
        let _ = state::with_state(|ui| ui.default_layout_hidden = true);
    }
}

pub fn apply_bindings() {
    if state::with_state(|ui| ui.bindings_applied).unwrap_or(true) {
        return;
    }
    let _ = spring::send_commands("unbindall", "");
    for binding in [
        "Any+esc reloadforce",
        "Any+pause  pause",
        "Any+p  pause",
        "Alt+b  debug",
        "Alt+v  debugcolvol",
        "f12 screenshot",
    ] {
        let _ = spring::send_commands("bind", binding);
    }
    let _ = spring::send_commands("unbindkeyset", "enter chat");
    let _ = spring::send_commands("unbind", "enter");
    let _ = spring::send_commands("unbindaction", "chat");
    let _ = state::with_state(|ui| ui.bindings_applied = true);
}

pub fn render(width: i32, height: i32) {
    let context = state::with_state(|ui| {
        ui.view_width = width;
        ui.view_height = height;
        ui.context
    })
    .unwrap_or(0);
    if context == 0 {
        return;
    }
    let _ = spring::context_set_dimensions(context, width, height);
}

fn create_context() -> Option<u64> {
    let context = match spring::create_rml_context("gravitas-wasm") {
        Ok(context) => context,
        Err(error) => {
            diagnostics::warning(&format!(
                "RmlUi context creation failed (error code {})",
                error.code
            ));
            return None;
        }
    };
    if !context.success || context.context_handle == 0 {
        diagnostics::warning("RmlUi context creation returned no context");
        return None;
    }
    Some(context.context_handle)
}

fn load_document(context: u64) -> Option<u64> {
    let document = match spring::load_rml_document(context, "LuaUI/wasm/gravitas.rml") {
        Ok(document) => document,
        Err(error) => {
            diagnostics::warning(&format!(
                "RmlUi document load failed (error code {})",
                error.code
            ));
            return None;
        }
    };
    if !document.success || document.document_handle == 0 {
        diagnostics::warning("RmlUi document load returned no document");
        return None;
    }
    let document = document.document_handle;
    let options = spring::owned::rml_ui::RmlDocumentShowOptions {
        modal: None,
        focus: Some(FOCUS_NONE),
    };
    match spring::rml_ui::document_show(document, options) {
        Ok(true) => Some(document),
        Ok(false) => {
            diagnostics::warning("RmlUi document show returned false");
            None
        }
        Err(error) => {
            diagnostics::warning(&format!(
                "RmlUi document show failed (error code {})",
                error.code
            ));
            None
        }
    }
}
