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
            spring::log_warning!(
                crate::diagnostics::LOG_SECTION,
                "RmlUi readiness query failed (error code {})",
                error.code
            );
            return;
        }
    }
    let Some(context) = create_context() else {
        return;
    };
    let Some(bindings) = model::create(context) else {
        spring::log_warning!(
            crate::diagnostics::LOG_SECTION,
            "RmlUi data-model creation failed"
        );
        let _ = spring::remove_context(context);
        return;
    };
    if !load_document(context) {
        let _ = spring::remove_context(context);
        return;
    }
    hud.context = context;
    hud.bindings = Some(bindings);
    if let Some(status) = hud.status {
        super::status::show(hud, status);
    }
    super::overlays::show(hud);
    diagnostics::info("Gravitas RmlUi HUD initialized");
}

pub fn shutdown(hud: &mut Hud) {
    if hud.context != 0 {
        let _ = spring::remove_context(hud.context);
        hud.context = 0;
        hud.bindings = None;
    }
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
            spring::log_warning!(
                crate::diagnostics::LOG_SECTION,
                "RmlUi context creation failed (error code {})",
                error.code
            );
            return None;
        }
    };
    if !context.success || context.context_handle == 0 {
        spring::log_warning!(
            crate::diagnostics::LOG_SECTION,
            "RmlUi context creation returned no context"
        );
        return None;
    }
    Some(context.context_handle)
}

fn load_document(context: u64) -> bool {
    let document = match spring::load_rml_document(context, "LuaUI/wasm/gravitas.rml") {
        Ok(document) => document,
        Err(error) => {
            spring::log_warning!(
                crate::diagnostics::LOG_SECTION,
                "RmlUi document load failed (error code {})",
                error.code
            );
            return false;
        }
    };
    if !document.success || document.document_handle == 0 {
        spring::log_warning!(
            crate::diagnostics::LOG_SECTION,
            "RmlUi document load returned no document"
        );
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
            spring::log_warning!(
                crate::diagnostics::LOG_SECTION,
                "RmlUi document show returned false"
            );
            false
        }
        Err(error) => {
            spring::log_warning!(
                crate::diagnostics::LOG_SECTION,
                "RmlUi document show failed (error code {})",
                error.code
            );
            false
        }
    }
}
