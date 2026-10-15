use spring::commands;
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
        if ctx.global().restart.wiping() || !crate::defs::get().unit(event.unit_def_id).barrel {
            return;
        }
        let Some(position) = crate::units::position(event.unit_id) else {
            return;
        };
        let barrel = event.unit_id;
        commands::run(move || effects::explode_barrel(barrel, position));
    }
}
