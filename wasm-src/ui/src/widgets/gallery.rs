//! Gallery mode, for the verification galleries
//! (`verification/fixtures/gallery/gallery.lua` in springcabal-games).  They
//! set the `gallery` modoption, place the camera themselves and capture single
//! units and effects, so only the widgets that draw what a player sees in the
//! world stay: no HUD, RmlUi document, input, camera or sound.

use spring_addons::register_widgets;
use spring_addons::ui::{HideDefaultInterface, WidgetHandler};

use super::*;
use crate::state::Global;

/// Whether the `gallery` modoption is set.
pub fn active() -> bool {
    spring::game_options::get_mod_option("gallery")
        .ok()
        .flatten()
        .is_some_and(|mode| !mode.is_empty())
}

/// The fire, sparks and smoke the rules report, and the map edge grid. The
/// health bars are HUD, so they stay out.
pub fn setup(handler: &mut WidgetHandler<Global>) {
    register_widgets!(
        handler,
        [
            HideDefaultInterface::new(setup::HIDDEN_INTERFACE),
            // Before the fire widget: it reads the frame this records.
            GameClockWidget::default(),
            GfxRulesEffectsWidget,
            GfxFireWidget::new(),
            GfxMapEdgeWidget::default(),
        ]
    );
    spring::log_notice!(
        "gravitas-ui",
        "gallery mode: HideDefaultInterface GameClock GfxRulesEffects GfxFire GfxMapEdge"
    );
}
