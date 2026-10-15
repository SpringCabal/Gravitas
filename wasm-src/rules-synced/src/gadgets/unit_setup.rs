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
        unit_scripts::attach_existing();
    }

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
        units::set_rules_params(unit_id, ctx.global().pending_rules.take());
        walls::configure(unit_id, unit_def_id);
        shapes::apply(unit_id, unit_def_id);
        shapes::configure_effect(unit_id, unit_def_id);
    }
}
