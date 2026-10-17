use spring_addons::ui::{HideDefaultInterface, WidgetHandler};
use springcabal::verification::WorldWidgets;

use super::*;
use crate::state::Global;

pub fn setup(spring: &spring::Spring<'_, spring::Ui>, handler: &mut WidgetHandler<Global>) {
    WorldWidgets::new(handler)
        .widget(HideDefaultInterface::new(setup::HIDDEN_INTERFACE))
        .widget(GameClockWidget::default())
        .widget(GfxRulesEffectsWidget)
        .widget(GfxFireWidget::new())
        .widget(GfxMapEdgeWidget::default())
        .announce(spring, crate::diagnostics::LOG_SECTION);
}
