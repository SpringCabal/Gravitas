use spring_addons::AddonContext;
use spring_addons::ui::Widget;

use crate::selection;
use crate::state::Global;

#[derive(Default)]
pub struct GuiUnselectableWidget;

impl GuiUnselectableWidget {
    pub fn new() -> Self {
        Self
    }
}

impl Widget<Global> for GuiUnselectableWidget {
    fn name(&self) -> &'static str {
        "GuiUnselectable"
    }

    fn init(&self, _ctx: &AddonContext<'_, Global>) {
        selection::configure_all();
    }

    fn unit_created(
        &self,
        _ctx: &AddonContext<'_, Global>,
        unit_id: i32,
        unit_def_id: i32,
        _unit_team: i32,
        _builder_id: i32,
    ) {
        selection::configure(unit_id, unit_def_id);
    }

    fn unit_entered_los(
        &self,
        _ctx: &AddonContext<'_, Global>,
        unit_id: i32,
        unit_def_id: i32,
        _unit_team: i32,
        _ally_team: i32,
    ) {
        selection::configure(unit_id, unit_def_id);
    }
}
