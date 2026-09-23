use spring_addons::AddonContext;
use spring_addons::ui::Widget;

use crate::mapedge::MapEdge;
use crate::state::Global;

#[derive(Default)]
pub struct GfxMapEdgeWidget {
    edge: MapEdge,
}

impl Widget<Global> for GfxMapEdgeWidget {
    fn name(&self) -> &'static str {
        "GfxMapEdge"
    }

    fn draw_world_pre_unit(&self, _ctx: &AddonContext<'_, Global>) {
        self.edge.draw();
    }
}
