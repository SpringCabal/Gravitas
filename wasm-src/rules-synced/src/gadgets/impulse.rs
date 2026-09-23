use spring_addons::AddonContext;
use spring_addons::rules::{Gadget, UnitDestroyedEvent, UnitPreDamagedEvent};

use crate::impulse;
use crate::state::Global;

/// The gravity guns push and pull what they hit.
pub struct ImpulseGadget;

impl Gadget<Global> for ImpulseGadget {
    fn name(&self) -> &'static str {
        "Impulse"
    }

    fn game_frame(&self, ctx: &AddonContext<'_, Global>, _frame: i32) {
        let pending = ctx.global().impulses.access_mut().take();
        impulse::apply(pending);
    }

    fn unit_pre_damaged(
        &self,
        ctx: &AddonContext<'_, Global>,
        event: &UnitPreDamagedEvent,
    ) -> Option<spring::DamageResult> {
        ctx.global().impulses.access_mut().damaged(
            event.unit_id,
            event.unit_def_id,
            event.weapon_def_id,
            event.attacker_id,
        );
        None
    }

    fn unit_destroyed(&self, ctx: &AddonContext<'_, Global>, event: &UnitDestroyedEvent) {
        ctx.global().impulses.access_mut().forget(event.unit_id);
    }
}
