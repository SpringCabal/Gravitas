use spring_addons::AddonContext;
use spring_addons::rules::Gadget;

use crate::state::Global;
use crate::{effects, shapes, unit_scripts, units, walls};

/// Gives each new unit its unit script, rules params (the def's, then the
/// scenario's), shape and blocking.
#[derive(Default)]
pub struct UnitSetupGadget;

impl Gadget<Global> for UnitSetupGadget {
    fn name(&self) -> &'static str {
        "UnitSetup"
    }

    /// Units that outlive a rules reload get their scripts here.
    fn init(&self, _ctx: &AddonContext<'_, Global>) {
        unit_scripts::attach_existing();
    }

    /// Wake the scripts' sleeping tasks where Lua does: in `GameFrame`,
    /// before the units update and aim.
    fn game_frame(&self, _ctx: &AddonContext<'_, Global>, frame: i32) {
        unit_scripts::game_frame(frame);
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
