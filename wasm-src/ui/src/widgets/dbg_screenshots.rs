#[cfg(feature = "headless-integration")]
use gravitas_shared::e2e::Request;

#[cfg(feature = "headless-integration")]
use spring_addons::AddonContext;
use spring_addons::ui::Widget;

use crate::state::Global;

pub struct DbgScreenshotsWidget;

impl Widget<Global> for DbgScreenshotsWidget {
    fn name(&self) -> &'static str {
        "DbgScreenshots"
    }

    #[cfg(feature = "headless-integration")]
    fn recv_from_synced(&self, ctx: &AddonContext<'_, Global, spring::Ui>, message: &[u8]) {
        let spring = ctx.spring();
        let global = ctx.global();
        let Some(request) = core::str::from_utf8(message).ok().and_then(Request::parse) else {
            return;
        };
        match request {
            Request::Shot(name) => {
                #[cfg(feature = "e2e")]
                springcabal::shots::screenshot(spring, name);
                springcabal::message::send_to_rules(
                    spring,
                    &gravitas_shared::e2e::shot_taken(name),
                );
            }
            Request::Done => {
                let _ = spring.messages().send_commands("quitforce", "");
            }
            Request::Restart => crate::widgets::gui_hud::hud::restart(spring, global),
            Request::PointerOff => global.scripted_pointer.set(None),
            Request::Pointer {
                target: [x, y, z],
                jump,
            } => global.scripted_pointer.set(Some(ScriptedPointer {
                target: spring::Float3 { x, y, z },
                jump,
            })),
            Request::Look { .. } => {}
        }
    }
}

#[cfg(feature = "headless-integration")]
#[derive(Clone, Copy)]
pub struct ScriptedPointer {
    pub target: spring::Float3,
    pub jump: bool,
}
