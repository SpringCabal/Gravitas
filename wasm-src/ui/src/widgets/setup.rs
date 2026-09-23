use spring_addons::register_widgets;
use spring_addons::ui::WidgetHandler;

use super::*;
use crate::state::Global;

pub fn setup(handler: &mut WidgetHandler<Global>) {
    register_widgets!(
        handler,
        [
            // First: the others read the frame it records in Update.
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
