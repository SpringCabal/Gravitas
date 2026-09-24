use core::cell::{Cell, RefCell};

use alloc::format;

use spring_addons::AddonContext;
use spring_addons::rules::{Gadget, UnitDestroyedEvent};

use crate::player::{self, Player};
use crate::state::Global;
use crate::{diagnostics, jump, mission};

const ENEMY_ALLY_TEAM: i32 = 1;

/// Takes the ui module's input and drives the player unit with it: moving,
/// aiming, holding fire and jumping. The game is lost with that unit.
#[derive(Default)]
pub struct PlayerGadget {
    player: RefCell<Player>,
    /// The last refusal logged, so a held jump key logs it once.
    last_refusal: Cell<Option<&'static str>>,
}

impl Gadget<Global> for PlayerGadget {
    fn name(&self) -> &'static str {
        "Player"
    }

    fn handle_lua_msg(
        &self,
        ctx: &AddonContext<'_, Global>,
        _player_id: i32,
        _script: i32,
        _mode: i32,
        data: &[u8],
    ) {
        let global = ctx.global();
        if data == gravitas_shared::RESTART.as_bytes() {
            global.restart.request();
            return;
        }
        if let Some(command) = player::parse(data) {
            player::accept(&global.player_input, command);
        }
    }

    fn unit_created(
        &self,
        ctx: &AddonContext<'_, Global>,
        unit_id: i32,
        unit_def_id: i32,
        team: i32,
        _builder: i32,
    ) {
        let adopted = self.player.borrow_mut().adopt(unit_id, unit_def_id, team);
        if adopted {
            ctx.global().player_unit.set(Some(unit_id));
            player::disable_auto_fire(unit_id);
        }
    }

    /// Losing the player's unit loses the game. Other Gravits (the e2e arena
    /// has several) are just units.
    fn unit_destroyed(&self, ctx: &AddonContext<'_, Global>, event: &UnitDestroyedEvent) {
        let global = ctx.global();
        let lost = {
            let mut player = self.player.borrow_mut();
            let lost = player.unit_id() == Some(event.unit_id);
            player.release(event.unit_id);
            global.player_unit.set(player.unit_id());
            lost
        };
        if lost && !global.restart.wiping() {
            mission::win(global, &[ENEMY_ALLY_TEAM]);
        }
    }

    fn game_frame(&self, ctx: &AddonContext<'_, Global>, frame: i32) {
        let global = ctx.global();
        let (unit_id, orders, jump) = {
            let mut player = self.player.borrow_mut();
            let orders = player.update(&global.player_input, frame);
            (player.unit_id(), orders, player.take_jump_request())
        };
        if let Some(unit_id) = unit_id {
            player::apply(unit_id, orders, frame);
        }
        if let Some((unit_id, x, y, z)) = jump {
            self.jump(global, unit_id, x, y, z);
        }
    }
}

impl PlayerGadget {
    fn jump(&self, global: &Global, unit_id: i32, x: f32, y: f32, z: f32) {
        let refusal = match jump::start(global, unit_id, x, y, z) {
            jump::Outcome::Started => None,
            jump::Outcome::OutOfRange => {
                // Walking closer; try again next frame.
                self.player.borrow_mut().retry_jump(x, y, z);
                None
            }
            outcome => Some(outcome.reason()),
        };
        if self.last_refusal.replace(refusal) != refusal
            && let Some(reason) = refusal
        {
            diagnostics::info(&format!("jump refused: {reason}"));
        }
    }
}
