use alloc::format;

use crate::diagnostics;

use super::{Hud, model};

const FOCUS_NONE: i32 = 0;

pub fn initialize(hud: &mut Hud) {
    if hud.context != 0 {
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
    let Some(bindings) = model::create(context) else {
        diagnostics::warning("RmlUi data-model creation failed");
        return;
    };
    if !load_document(context) {
        return;
    }
    hud.context = context;
    hud.bindings = Some(bindings);
    let status = core::mem::take(&mut hud.status);
    super::status::show(hud, &status);
    hud.status = status;
    diagnostics::info("Gravitas RmlUi HUD initialized");
}

/// Replace the engine's key bindings with the few the game keeps.
pub fn apply_bindings(hud: &mut Hud) {
    if hud.keys_bound {
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
    hud.keys_bound = true;
}

pub fn render(hud: &Hud, width: i32, height: i32) {
    if hud.context != 0 {
        let _ = spring::context_set_dimensions(hud.context, width, height);
    }
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

fn load_document(context: u64) -> bool {
    let document = match spring::load_rml_document(context, "LuaUI/wasm/gravitas.rml") {
        Ok(document) => document,
        Err(error) => {
            diagnostics::warning(&format!(
                "RmlUi document load failed (error code {})",
                error.code
            ));
            return false;
        }
    };
    if !document.success || document.document_handle == 0 {
        diagnostics::warning("RmlUi document load returned no document");
        return false;
    }
    let document = document.document_handle;
    let options = spring::owned::rml_ui::RmlDocumentShowOptions {
        modal: None,
        focus: Some(FOCUS_NONE),
    };
    match spring::rml_ui::document_show(document, options) {
        Ok(true) => true,
        Ok(false) => {
            diagnostics::warning("RmlUi document show returned false");
            false
        }
        Err(error) => {
            diagnostics::warning(&format!(
                "RmlUi document show failed (error code {})",
                error.code
            ));
            false
        }
    }
}
