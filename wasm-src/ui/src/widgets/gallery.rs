//! Gallery mode (`springcabal::gallery`): only the widgets that draw what a
//! player sees in the world.

use spring_addons::ui::{HideDefaultInterface, WidgetHandler};
use springcabal::gallery::Gallery;

use super::*;
use crate::state::Global;

/// The fire, sparks and smoke the rules report, and the map edge grid. The
/// health bars are HUD, so they stay out.
pub fn setup(handler: &mut WidgetHandler<Global>) {
    Gallery::new(handler)
        .widget(HideDefaultInterface::new(setup::HIDDEN_INTERFACE))
        // Before the fire widget: it reads the frame this records.
        .widget(GameClockWidget::default())
        .widget(GfxRulesEffectsWidget)
        .widget(GfxFireWidget::new())
        .widget(GfxMapEdgeWidget::default())
        .announce(crate::diagnostics::LOG_SECTION);
}
