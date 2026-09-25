//! The in-game cases' requests to the ui module (only with the
//! `headless-integration` feature): screenshots (taken only in the `e2e`
//! build), a scripted pointer for the input widget, and the HUD's restart.
//! Each goes through the same path as the real thing, and a screenshot is
//! reported back to the rules. The camera widget handles
//! the scenario's camera moves.

#[cfg(feature = "headless-integration")]
use gravitas_shared::e2e::Request;

use crate::state::Global;

/// Stands in for the mouse: where it points, and whether the middle (jump)
/// button is held.
#[cfg(feature = "headless-integration")]
#[derive(Clone, Copy)]
pub struct ScriptedPointer {
    pub target: spring::types::Float3,
    pub jump: bool,
}

#[cfg(feature = "headless-integration")]
pub fn handle(global: &Global, message: &[u8]) {
    let Some(request) = core::str::from_utf8(message).ok().and_then(Request::parse) else {
        return;
    };
    match request {
        Request::Shot(name) => {
            #[cfg(feature = "e2e")]
            springcabal::shots::screenshot(name);
            springcabal::message::send_to_rules(&gravitas_shared::e2e::shot_taken(name));
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

#[cfg(not(feature = "headless-integration"))]
pub fn handle(_global: &Global, _message: &[u8]) {}
