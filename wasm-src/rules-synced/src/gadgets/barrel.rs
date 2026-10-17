use spring_addons::AddonContext;
use spring_addons::rules::{Gadget, UnitDestroyedEvent};

use crate::effects;
use crate::state::Global;

pub struct BarrelGadget;

impl Gadget<Global> for BarrelGadget {
    fn name(&self) -> &'static str {
        "Barrel"
    }

    fn unit_destroyed(&self, ctx: &AddonContext<'_, Global>, event: &UnitDestroyedEvent) {
        let spring = ctx.spring();
        if ctx.global().restart.wiping() || !crate::defs::get(spring).unit(event.unit_def_id).barrel
        {
            return;
        }
        let Some(position) = crate::units::position(spring, event.unit_id) else {
            return;
        };
        let barrel = event.unit_id;
        crate::with_addon_context(|ctx| {
            ctx.commands()
                .run(move |spring| effects::explode_barrel(spring, barrel, position))
        });
    }
}
