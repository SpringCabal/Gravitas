use spring_addons::rules::Gadget;

use crate::state::{self, Global};
use crate::{player, restart};

const RESTART_MESSAGE: &[u8] = b"gravitas_restart";

#[derive(Default)]
pub struct InputGadget;

impl InputGadget {
    pub fn new() -> Self {
        Self
    }
}

impl Gadget<Global> for InputGadget {
    fn name(&self) -> &'static str {
        "Input"
    }

    fn handle_lua_msg(
        &self,
        _ctx: &spring_addons::AddonContext<'_, Global>,
        _player_id: i32,
        _script: i32,
        _mode: i32,
        data: &[u8],
    ) {
        if data == RESTART_MESSAGE {
            restart::request();
            return;
        }
        let Some(command) = player::parse(data) else {
            return;
        };
        let _ = state::with_world(|world| world.player.accept(command));
    }
}
