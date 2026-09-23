use spring_addons::AddonContext;
use spring_addons::rules::{Gadget, UnitDestroyedEvent};

use crate::state::Global;
use crate::{probe, terrain};

/// Levels the ground under walls: raised while a wall stands, flat once it is
/// gone.
pub struct TerrainGadget;

impl Gadget<Global> for TerrainGadget {
    fn name(&self) -> &'static str {
        "Terrain"
    }

    fn game_frame(&self, ctx: &AddonContext<'_, Global>, frame: i32) {
        let global = ctx.global();
        let due = global.terrain.access_mut().take_due(frame);
        terrain::level(due);
        probe::report_levelled(global, frame);
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
