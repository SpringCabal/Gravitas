use spring_addons::AddonContext;
use spring_addons::rules::Gadget;

use crate::state::Global;
use crate::{effects, shapes, unit_scripts, units, walls};

/// Gives each new unit its unit script, rules params (the def's, then the
/// scenario's), shape and blocking.
pub struct UnitSetupGadget;

impl Gadget<Global> for UnitSetupGadget {
    fn name(&self) -> &'static str {
        "UnitSetup"
    }

    /// Units that exist before the gadget (a rules reload) still need scripts.
    fn init(&self, _ctx: &AddonContext<'_, Global>) {
        for unit_id in spring::get_all_units().unwrap_or_default() {
            if let Ok(def) = spring::get_unit_def_id(unit_id) {
                unit_scripts::attach(unit_id, def.0);
            }
        }
    }

    fn unit_created(
        &self,
        ctx: &AddonContext<'_, Global>,
        unit_id: i32,
        unit_def_id: i32,
        _team: i32,
        _builder: i32,
    ) {
        unit_scripts::attach(unit_id, unit_def_id);
        effects::tag_on_create(unit_id, unit_def_id);
        // After the def's tags: the scenario can override them.
        units::set_rules_params(unit_id, ctx.global().pending_rules.take());
        walls::configure(unit_id, unit_def_id);
        shapes::apply(unit_id, unit_def_id);
        shapes::configure_effect(unit_id, unit_def_id);
    }
}
