use spring::default_interface::{
    CLOCK, CONSOLE, END_GRAPH, MAP_BORDER, MINIMAP, PLAYER_INFO, RESOURCE_BAR, TOOLTIP, UNIT_ICONS,
};
use spring_addons::register_widgets;
use spring_addons::ui::{HideDefaultInterface, WidgetHandler};

use super::*;
use crate::state::Global;

pub const HIDDEN_INTERFACE: u32 = CONSOLE
    | RESOURCE_BAR
    | TOOLTIP
    | CLOCK
    | PLAYER_INFO
    | MINIMAP
    | END_GRAPH
    | UNIT_ICONS
    | MAP_BORDER;

pub fn setup(handler: &mut WidgetHandler<Global>) {
    if springcabal::verification::world_only() {
        gallery::setup(handler);
        return;
    }
    register_widgets!(
        handler,
        [
            HideDefaultInterface::new(HIDDEN_INTERFACE),
            GameClockWidget::default(),
            GfxRulesEffectsWidget,
            DbgScreenshotsWidget,
            GuiHudWidget,
            GuiCameraWidget::default(),
            GuiInputWidget::default(),
            GuiUnselectableWidget::new(),
            GfxFireWidget::new(),
            snd_music::music(),
            GfxHealthbarsWidget::new(),
            GfxMapEdgeWidget::default(),
        ]
    );
}
