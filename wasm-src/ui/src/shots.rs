//! The e2e scenario's requests to the ui module (only with the `e2e`
//! feature): screenshots, a scripted pointer for the input widget, and the
//! HUD's restart. Each goes through the same path as the real thing, and a
//! taken screenshot is reported back to the rules.

use crate::state::Global;

/// Stands in for the mouse: where it points, and whether the middle (jump)
/// button is held.
#[cfg(feature = "e2e")]
#[derive(Clone, Copy)]
pub struct ScriptedPointer {
    pub target: spring::types::Float3,
    pub jump: bool,
}

#[cfg(feature = "e2e")]
pub fn handle(global: &Global, message: &[u8]) {
    let Ok(message) = core::str::from_utf8(message) else {
        return;
    };
    if let Some(request) = message.strip_prefix("GRAVITAS_SHOT|") {
        shot(request);
    } else if let Some(request) = message.strip_prefix("GRAVITAS_E2E|") {
        scripted(global, request);
    }
}

#[cfg(not(feature = "e2e"))]
pub fn handle(_global: &Global, _message: &[u8]) {}

#[cfg(feature = "e2e")]
fn shot(request: &str) {
    if let Some(name) = request.strip_prefix("shot|") {
        if spring::send_commands("screenshot", "").is_ok() {
            let _ = spring::send_lua_rules_msg(&alloc::format!("gravitas_e2e|shot|{name}"));
        }
    } else if request == "done" {
        let _ = spring::send_commands("quitforce", "");
    }
}

/// `pointer|x|y|z|jump`, `pointer|off` or `restart`.
#[cfg(feature = "e2e")]
fn scripted(global: &Global, request: &str) {
    if request == "restart" {
        crate::hud::restart(global);
        return;
    }
    if request == "pointer|off" {
        global.scripted_pointer.set(None);
        return;
    }
    let pointer = request.strip_prefix("pointer|").and_then(parse_pointer);
    global.scripted_pointer.set(pointer);
}

#[cfg(feature = "e2e")]
fn parse_pointer(fields: &str) -> Option<ScriptedPointer> {
    let mut fields = fields.split('|');
    let mut number = || {
        fields
            .next()?
            .parse::<f32>()
            .ok()
            .filter(|value| value.is_finite())
    };
    let target = spring::types::Float3 {
        x: number()?,
        y: number()?,
        z: number()?,
    };
    Some(ScriptedPointer {
        target,
        jump: fields.next()? == "1",
    })
}
