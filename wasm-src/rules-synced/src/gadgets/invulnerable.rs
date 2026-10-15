use spring_addons::AddonContext;
use spring_addons::rules::{Gadget, UnitPreDamagedEvent};

use crate::effects;
use crate::state::Global;

pub struct InvulnerableGadget;

impl Gadget<Global> for InvulnerableGadget {
    fn name(&self) -> &'static str {
        "Invulnerable"
    }

    fn unit_pre_damaged(
        &self,
        _ctx: &AddonContext<'_, Global>,
        event: &UnitPreDamagedEvent,
    ) -> Option<spring::DamageResult> {
        effects::blocks_damage(event.unit_id, event.paralyzer)
            .then(|| spring::DamageResult::unchanged(0.0))
    }
}
