use spring_addons::ui::{HideDefaultInterface, WidgetHandler};
use springcabal::verification::WorldWidgets;

use super::*;
use crate::state::Global;

pub fn setup(spring: &spring::Spring<'_, spring::Ui>, handler: &mut WidgetHandler<Global>) {
    let widgets = WorldWidgets::new(handler)
        .widget(HideDefaultInterface::new(setup::HIDDEN_INTERFACE))
        .widget(GameClockWidget::default())
        .widget(GfxRulesEffectsWidget)
        .widget(GfxFireWidget::new());
    // Keep gallery walls out of the isolated fire performance workload.
    let fire_perf = cfg!(feature = "fire-probe")
        && spring
            .game()
            .get_mod_option("fire_perf")
            .is_ok_and(|v| v.value == "1");
    if fire_perf {
        widgets.announce(spring, crate::diagnostics::LOG_SECTION);
    } else {
        widgets
            .widget(GfxMapEdgeWidget::default())
            .announce(spring, crate::diagnostics::LOG_SECTION);
    }
}
