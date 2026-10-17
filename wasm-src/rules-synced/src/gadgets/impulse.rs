pub(crate) mod behavior;

use spring_addons::AddonContext;
use spring_addons::rules::{Gadget, UnitDestroyedEvent, UnitPreDamagedEvent};

use crate::gadgets::impulse::behavior as impulse;
use crate::state::Global;

pub struct ImpulseGadget;

impl Gadget<Global> for ImpulseGadget {
    fn name(&self) -> &'static str {
        "Impulse"
    }

    fn game_frame(&self, ctx: &AddonContext<'_, Global>, _frame: i32) {
        let spring = ctx.spring();
        let pending = ctx.global().impulses.access_mut().take();
        impulse::apply(spring, pending);
    }

    fn unit_pre_damaged(
        &self,
        ctx: &AddonContext<'_, Global>,
        event: &UnitPreDamagedEvent,
    ) -> Option<spring::DamageResult> {
        let spring = ctx.spring();
        ctx.global().impulses.access_mut().damaged(
            spring,
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
