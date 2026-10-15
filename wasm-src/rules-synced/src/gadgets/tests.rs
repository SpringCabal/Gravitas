use spring_addons::AddonContext;
use spring_addons::rules::{Gadget, UnitDestroyedEvent};

use crate::state::Global;
use crate::tests;

#[derive(Default)]
pub struct TestsGadget {
    runner: tests::Runner,
}

impl Gadget<Global> for TestsGadget {
    fn name(&self) -> &'static str {
        "Tests"
    }

    fn game_frame(&self, ctx: &AddonContext<'_, Global>, frame: i32) {
        self.runner.game_frame(ctx.global(), frame);
    }

    fn handle_lua_msg(
        &self,
        ctx: &AddonContext<'_, Global>,
        _player_id: i32,
        _script: i32,
        _mode: i32,
        data: &[u8],
    ) {
        tests::note_message(ctx.global(), data);
    }

    fn unit_created(
        &self,
        ctx: &AddonContext<'_, Global>,
        unit_id: i32,
        unit_def_id: i32,
        _team: i32,
        _builder: i32,
    ) {
        tests::note_created(ctx.global(), unit_id, unit_def_id);
    }

    fn unit_destroyed(&self, ctx: &AddonContext<'_, Global>, event: &UnitDestroyedEvent) {
        tests::note_destroyed(ctx.global(), event.unit_id, event.unit_def_id);
    }
}
