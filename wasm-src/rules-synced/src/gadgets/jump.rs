pub mod behavior;
use spring_addons::AddonContext;
use spring_addons::rules::{Gadget, UnitDestroyedEvent};

use crate::state::Global;

pub struct JumpGadget;

impl Gadget<Global> for JumpGadget {
    fn name(&self) -> &'static str {
        "Jump"
    }

    fn game_frame(&self, ctx: &AddonContext<'_, Global>, frame: i32) {
        let global = ctx.global();
        let landed = behavior::fly(global);
        let mut immunity = global.fall_immunity.access_mut();
        for unit_id in landed {
            immunity.set_immunity(unit_id, frame + behavior::LANDING_IMMUNITY_FRAMES);
        }
    }

    fn unit_destroyed(&self, ctx: &AddonContext<'_, Global>, event: &UnitDestroyedEvent) {
        let global = ctx.global();
        global.jumps.access_mut().forget(event.unit_id);
        global.fall_immunity.access_mut().forget(event.unit_id);
    }
}
