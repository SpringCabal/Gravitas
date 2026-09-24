use spring::default_interface::{
    CLOCK, CONSOLE, END_GRAPH, MAP_BORDER, MINIMAP, PLAYER_INFO, RESOURCE_BAR, TOOLTIP, UNIT_ICONS,
};
use spring_addons::register_widgets;
use spring_addons::ui::{HideDefaultInterface, WidgetHandler};

use super::*;
use crate::state::Global;

/// The engine interface parts the Lua game hid; the FPS counter and speed
/// indicator stay, as there. The map edge grid replaces the border.
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
    if gallery::active() {
        gallery::setup(handler);
        return;
    }
    register_widgets!(
        handler,
        [
            HideDefaultInterface::new(HIDDEN_INTERFACE),
            // Before the fire widget: it reads the frame this records.
            GameClockWidget::default(),
            GfxRulesEffectsWidget,
            DbgScreenshotsWidget,
            // Before the camera: it records the player unit from the status.
            GuiHudWidget,
            GuiCameraWidget::default(),
            GuiInputWidget::default(),
            GuiUnselectableWidget::new(),
            GfxFireWidget::new(),
            SndMusicWidget::default(),
            GfxHealthbarsWidget::new(),
            GfxMapEdgeWidget::default(),
        ]
    );
}
