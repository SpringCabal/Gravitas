use spring_addons::ui::Widget;

use crate::mapedge;
use crate::state::Global;

#[derive(Default)]
pub struct GfxMapEdgeWidget;

impl GfxMapEdgeWidget {
    pub fn new() -> Self {
        Self
    }
}

impl Widget<Global> for GfxMapEdgeWidget {
    fn name(&self) -> &'static str {
        "GfxMapEdge"
    }

    fn draw_world_pre_unit(&self, _ctx: &spring_addons::AddonContext<'_, Global>) {
        mapedge::draw();
    }
}
