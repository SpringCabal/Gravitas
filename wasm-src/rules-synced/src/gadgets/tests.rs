use spring_addons::AddonContext;
use spring_addons::rules::{Gadget, UnitDestroyedEvent};

use crate::state::Global;
use crate::tests;

/// Runs the in-game e2e cases (only with the `e2e` feature).
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

    fn unit_destroyed(&self, ctx: &AddonContext<'_, Global>, event: &UnitDestroyedEvent) {
        tests::note_destroyed(ctx.global(), event.unit_id, event.unit_def_id);
    }
}
