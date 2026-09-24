use core::cell::Cell;

use spring_addons::AddonContext;
use spring_addons::rules::Gadget;

use crate::state::Global;
use crate::{effects, shapes, unit_scripts, units, walls};

/// Gives each new unit its unit script, rules params (the def's, then the
/// scenario's), shape and blocking.
#[derive(Default)]
pub struct UnitSetupGadget {
    started: Cell<bool>,
}

impl Gadget<Global> for UnitSetupGadget {
    fn name(&self) -> &'static str {
        "UnitSetup"
    }

    /// Units created before the game starts (or that survived a rules
    /// reload) get their scripts on the first frame. Before it `CusHost`
    /// reads the frame as `u32::MAX`, so a task that sleeps in `Create`
    /// would never wake (engine TODO item I).
    fn game_frame(&self, _ctx: &AddonContext<'_, Global>, _frame: i32) {
        if !self.started.replace(true) {
            unit_scripts::attach_existing();
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
        if self.started.get() {
            unit_scripts::attach(unit_id, unit_def_id);
        }
        effects::tag_on_create(unit_id, unit_def_id);
        // After the def's tags: the scenario can override them.
        units::set_rules_params(unit_id, ctx.global().pending_rules.take());
        walls::configure(unit_id, unit_def_id);
        shapes::apply(unit_id, unit_def_id);
        shapes::configure_effect(unit_id, unit_def_id);
    }
}
