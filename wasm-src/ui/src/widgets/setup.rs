use spring::default_interface::{
    CLOCK, CONSOLE, END_GRAPH, MAP_BORDER, MINIMAP, PLAYER_INFO, RESOURCE_BAR, TOOLTIP, UNIT_ICONS,
};
use spring_addons::register_widgets;
use spring_addons::ui::{HideDefaultInterface, WidgetHandler};

use super::*;
use crate::state::Global;

pub fn setup(handler: &mut WidgetHandler<Global>) {
    register_widgets!(
        handler,
        [
            // The engine interface parts the Lua game hid; the FPS counter
            // and speed indicator stay, as there. The map edge grid replaces
            // the border.
            HideDefaultInterface::new(
                CONSOLE
                    | RESOURCE_BAR
                    | TOOLTIP
                    | CLOCK
                    | PLAYER_INFO
                    | MINIMAP
                    | END_GRAPH
                    | UNIT_ICONS
                    | MAP_BORDER
            ),
            // Before the fire widget: it reads the frame this records.
            GameClockWidget::default(),
            ApiSyncWidget,
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
