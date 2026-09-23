use spring_addons::AddonContext;
use spring_addons::rules::{Gadget, UnitDestroyedEvent};

use crate::state::Global;
use crate::tests;

/// Runs the in-game e2e cases (only with the `e2e` feature).
pub struct TestsGadget;

impl Gadget<Global> for TestsGadget {
    fn name(&self) -> &'static str {
        "Tests"
    }

    fn game_frame(&self, ctx: &AddonContext<'_, Global>, frame: i32) {
        tests::game_frame(ctx.global(), frame);
    }

    fn unit_destroyed(&self, _ctx: &AddonContext<'_, Global>, event: &UnitDestroyedEvent) {
        tests::note_destroyed(event.unit_id, event.unit_def_id);
    }
}
