use spring_addons::AddonContext;
use spring_addons::ui::Widget;

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
        configure_all();
    }

    fn unit_created(
        &self,
        _ctx: &AddonContext<'_, Global>,
        unit_id: i32,
        unit_def_id: i32,
        _unit_team: i32,
        _builder_id: i32,
    ) {
        configure(unit_id, unit_def_id);
    }

    fn unit_entered_los(
        &self,
        _ctx: &AddonContext<'_, Global>,
        unit_id: i32,
        unit_def_id: i32,
        _unit_team: i32,
        _ally_team: i32,
    ) {
        configure(unit_id, unit_def_id);
    }
}

fn configure(unit_id: i32, unit_def_id: i32) {
    let _ = spring::unsynced_ctrl::set_unit_no_select(unit_id, true);
    if crate::defs::unit(unit_def_id).effect {
        let _ = spring::unsynced_ctrl::set_unit_no_draw(unit_id, true);
    }
    let _ = spring::selection::select_unit_array(&[], false);
}

fn configure_all() {
    for unit_id in spring::get_all_units().unwrap_or_default() {
        if let Ok(unit_def_id) = spring::units_info::get_unit_def_id(unit_id) {
            configure(unit_id, unit_def_id);
        }
    }
}
