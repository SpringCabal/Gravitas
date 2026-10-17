pub(crate) mod walls;

pub(crate) mod shapes;

use spring_addons::AddonContext;
use spring_addons::rules::Gadget;

use crate::effects;
use crate::state::Global;
use crate::unit_scripts;
use crate::units;

#[derive(Default)]
pub struct UnitSetupGadget;

impl Gadget<Global> for UnitSetupGadget {
    fn name(&self) -> &'static str {
        "UnitSetup"
    }

    fn init(&self, _ctx: &AddonContext<'_, Global>) {
        let spring = _ctx.spring();
        unit_scripts::attach_existing(spring);
    }

    fn game_frame(&self, _ctx: &AddonContext<'_, Global>, frame: i32) {
        let spring = _ctx.spring();
        unit_scripts::game_frame(spring, frame);
    }

    fn unit_created(
        &self,
        ctx: &AddonContext<'_, Global>,
        unit_id: i32,
        unit_def_id: i32,
        _team: i32,
        _builder: i32,
    ) {
        let spring = ctx.spring();
        unit_scripts::attach(spring, unit_id, unit_def_id);
        effects::tag_on_create(spring, unit_id, unit_def_id);
        units::set_rules_params(spring, unit_id, ctx.global().pending_rules.take());
        walls::configure(spring, unit_id, unit_def_id);
        shapes::apply(spring, unit_id, unit_def_id);
        shapes::configure_effect(spring, unit_id, unit_def_id);
    }
}
