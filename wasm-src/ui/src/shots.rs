//! The e2e scenario's requests to the ui module (only with the `e2e`
//! feature): screenshots, a scripted pointer for the input widget, and the
//! HUD's restart. Each goes through the same path as the real thing, and a
//! taken screenshot is reported back to the rules. The camera widget handles
//! the scenario's camera moves.

#[cfg(feature = "e2e")]
use gravitas_shared::e2e::Request;

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
    let Some(request) = core::str::from_utf8(message).ok().and_then(Request::parse) else {
        return;
    };
    match request {
        Request::Shot(name) => {
            springcabal::shots::screenshot(name);
            let _ = spring::send_lua_rules_msg(&gravitas_shared::e2e::shot_taken(name));
        }
        Request::Done => {
            let _ = spring::send_commands("quitforce", "");
        }
        Request::Restart => crate::hud::restart(global),
        Request::PointerOff => global.scripted_pointer.set(None),
        Request::Pointer {
            target: [x, y, z],
            jump,
        } => global.scripted_pointer.set(Some(ScriptedPointer {
            target: spring::types::Float3 { x, y, z },
            jump,
        })),
        Request::Look { .. } => {}
    }
}

#[cfg(not(feature = "e2e"))]
pub fn handle(_global: &Global, _message: &[u8]) {}
