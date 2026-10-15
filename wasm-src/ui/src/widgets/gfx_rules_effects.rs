use gravitas_shared::{Effect, EffectKind};
use spring_addons::AddonContext;
use spring_addons::ui::Widget;

use crate::state::Global;

pub struct GfxRulesEffectsWidget;

impl Widget<Global> for GfxRulesEffectsWidget {
    fn name(&self) -> &'static str {
        "GfxRulesEffects"
    }

    fn recv_from_synced(&self, ctx: &AddonContext<'_, Global>, message: &[u8]) {
        if let Some(effect) = core::str::from_utf8(message).ok().and_then(Effect::parse) {
            handle_effect(ctx.global(), effect);
        }
    }
}

fn handle_effect(global: &Global, effect: Effect) {
    let [x, y, z] = effect.position;
    if !in_view(x, y, z, effect.magnitude()) {
        return;
    }
    let position = spring::types::Float3 { x, y, z };
    let mut fire = global.fire.access_mut();
    match effect.kind {
        EffectKind::Burn {
            radius,
            strength,
            velocity,
        } => crate::widgets::gfx_fire::fire::burn(&mut fire, position, radius, strength, velocity),
        EffectKind::Flame { radius } => {
            crate::widgets::gfx_fire::fire::flame(&mut fire, position, radius)
        }
        EffectKind::Arc { radius } => {
            crate::widgets::gfx_fire::fire::electric_field(&mut fire, position, radius)
        }
        EffectKind::Spark {
            intensity,
            velocity,
            spread,
        } => {
            let position = spring::types::Float3 {
                x: position.x + spread * signed_random(),
                y: position.y + spread * signed_random(),
                z: position.z + spread * signed_random(),
            };
            crate::widgets::gfx_fire::fire::electric_spark(
                &mut fire, position, intensity, velocity,
            );
        }
    }
}

fn in_view(x: f32, y: f32, z: f32, magnitude: f32) -> bool {
    let centre = spring::types::Float3 { x, y, z };
    spring::display::is_sphere_in_view(centre, magnitude.max(VIEW_RADIUS)).unwrap_or(true)
}

const VIEW_RADIUS: f32 = 150.0;

fn signed_random() -> f32 {
    spring::random::unsynced::range(-1.0, 1.0)
}
