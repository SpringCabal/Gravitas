use spring_addons::register_widgets;
use spring_addons::ui::WidgetHandler;

use super::*;
use crate::state::Global;

pub fn setup(handler: &mut WidgetHandler<Global>) {
    register_widgets!(
        handler,
        [
            ApiSyncWidget::new(),
            GuiHudWidget::new(),
            GuiCameraWidget::new(),
            GuiInputWidget::new(),
            GuiUnselectableWidget::new(),
            GfxFireWidget::new(),
            GuiHudPauseWidget::new(),
            SndMusicWidget::new(),
            GfxHealthbarsWidget::new(),
            GfxMapEdgeWidget::new(),
        ]
    );
}
