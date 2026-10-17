pub mod behavior;
use core::cell::{Cell, RefCell};

use alloc::format;

use spring_addons::AddonContext;
use spring_addons::rules::{Gadget, UnitDestroyedEvent};

use self::behavior::Player;
use crate::diagnostics;
use crate::gadgets::jump::behavior as jump;
use crate::gadgets::mission::behavior as mission;
use crate::state::Global;

const ENEMY_ALLY_TEAM: i32 = 1;

#[derive(Default)]
pub struct PlayerGadget {
    player: RefCell<Player>,
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
        if let Some(command) = behavior::parse(data) {
            behavior::accept(&global.player_input, command);
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
        let spring = ctx.spring();
        let adopted = self
            .player
            .borrow_mut()
            .adopt(spring, unit_id, unit_def_id, team);
        if adopted {
            ctx.global().player_unit.set(Some(unit_id));
            behavior::disable_auto_fire(spring, unit_id);
        }
    }

    fn unit_destroyed(&self, ctx: &AddonContext<'_, Global>, event: &UnitDestroyedEvent) {
        let spring = ctx.spring();
        let global = ctx.global();
        let lost = {
            let mut player = self.player.borrow_mut();
            let lost = player.unit_id() == Some(event.unit_id);
            player.release(event.unit_id);
            global.player_unit.set(player.unit_id());
            lost
        };
        if lost && !global.restart.wiping() {
            mission::win(spring, global, &[ENEMY_ALLY_TEAM]);
        }
    }

    fn game_frame(&self, ctx: &AddonContext<'_, Global>, frame: i32) {
        let spring = ctx.spring();
        let global = ctx.global();
        let mut player = self.player.borrow_mut();
        let orders = player.update(spring, &global.player_input, frame);
        if let Some(unit_id) = player.unit_id() {
            behavior::apply(spring, unit_id, orders, frame);
        }
        if let Some((unit_id, x, y, z)) = player.take_jump_request() {
            self.jump(spring, global, &mut player, unit_id, x, y, z);
        }
    }
}

impl PlayerGadget {
    fn jump(
        &self,
        spring: &spring::Spring<'_, spring::RulesSynced>,
        global: &Global,
        player: &mut Player,
        unit_id: i32,
        x: f32,
        y: f32,
        z: f32,
    ) {
        let refusal = match jump::start(spring, global, unit_id, x, y, z) {
            jump::Outcome::Started => None,
            jump::Outcome::OutOfRange => {
                player.retry_jump(x, y, z);
                None
            }
            outcome => Some(outcome.reason()),
        };
        if self.last_refusal.replace(refusal) != refusal
            && let Some(reason) = refusal
        {
            diagnostics::info(spring, &format!("jump refused: {reason}"));
        }
    }
}
