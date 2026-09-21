use spring_addons::rules::{Gadget, UnitDestroyedEvent, UnitPreDamagedEvent};

use crate::lifecycle;
use crate::state::Global;
use crate::unit_scripts;

#[derive(Default)]
pub struct LifecycleGadget {
    swept: core::cell::Cell<bool>,
}

impl LifecycleGadget {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Gadget<Global> for LifecycleGadget {
    fn name(&self) -> &'static str {
        "Lifecycle"
    }

    fn game_frame(&self, _ctx: &spring_addons::AddonContext<'_, Global>, _frame: i32) {
        if self.swept.replace(true) {
            return;
        }
        let Ok(units) = spring::get_all_units() else {
            return;
        };
        for unit in units {
            if let Ok(def) = spring::get_unit_def_id(unit) {
                unit_scripts::queue_unit_created(unit, def.0);
            }
        }
    }

    fn unit_created(
        &self,
        _ctx: &spring_addons::AddonContext<'_, Global>,
        unit: i32,
        def: i32,
        team: i32,
        builder: i32,
    ) {
        unit_scripts::queue_unit_created(unit, def);
        lifecycle::unit_created(unit, def, team, builder);
    }

    fn unit_destroyed(
        &self,
        _ctx: &spring_addons::AddonContext<'_, Global>,
        event: &UnitDestroyedEvent,
    ) {
        lifecycle::unit_destroyed(
            event.unit_id,
            event.unit_def_id,
            event.unit_team,
            event.attacker_id,
            event.attacker_def_id,
            event.attacker_team,
            event.weapon_def_id,
        );
    }

    fn unit_pre_damaged(
        &self,
        _ctx: &spring_addons::AddonContext<'_, Global>,
        event: &UnitPreDamagedEvent,
    ) -> Option<spring::DamageResult> {
        Some(lifecycle::unit_pre_damaged(
            event.unit_id,
            event.unit_def_id,
            event.unit_team,
            event.damage,
            event.paralyzer,
            event.weapon_def_id,
            event.projectile_id,
            event.attacker_id,
            event.attacker_def_id,
            event.attacker_team,
        ))
    }
}
