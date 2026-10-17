pub(crate) mod behavior;

use spring_addons::AddonContext;
use spring_addons::rules::{Gadget, UnitDestroyedEvent};

use crate::gadgets::terrain::behavior as terrain;
use crate::state::Global;

pub struct TerrainGadget;

impl Gadget<Global> for TerrainGadget {
    fn name(&self) -> &'static str {
        "Terrain"
    }

    fn game_frame(&self, ctx: &AddonContext<'_, Global>, frame: i32) {
        let spring = ctx.spring();
        let due = ctx.global().terrain.access_mut().take_due(spring, frame);
        terrain::level(spring, due);
    }

    fn unit_created(
        &self,
        ctx: &AddonContext<'_, Global>,
        unit_id: i32,
        unit_def_id: i32,
        _team: i32,
        _builder: i32,
    ) {
        let mut terrain = ctx.global().terrain.access_mut();
        terrain.defer(unit_id, unit_def_id, 1.0);
    }

    fn unit_destroyed(&self, ctx: &AddonContext<'_, Global>, event: &UnitDestroyedEvent) {
        let mut terrain = ctx.global().terrain.access_mut();
        terrain.defer(event.unit_id, event.unit_def_id, 0.0);
    }
}
