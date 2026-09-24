use gravitas_shared::{Effect, EffectKind};

use crate::state::Global;

/// Draw a fire or electric effect the rules module sent, if it can be seen.
pub fn handle(global: &Global, effect: Effect) {
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
        } => crate::fire::burn(&mut fire, position, radius, strength, velocity),
        EffectKind::Flame { radius } => crate::fire::flame(&mut fire, position, radius),
        EffectKind::Arc { radius } => crate::fire::electric_field(&mut fire, position, radius),
        EffectKind::Spark {
            intensity,
            velocity,
            spread,
        } => {
            // Like the Lua effect, every spark starts at its own point inside
            // a cube around the unit.
            let position = spring::types::Float3 {
                x: position.x + spread * signed_random(),
                y: position.y + spread * signed_random(),
                z: position.z + spread * signed_random(),
            };
            crate::fire::electric_spark(&mut fire, position, intensity, velocity);
        }
    }
}

fn in_view(x: f32, y: f32, z: f32, magnitude: f32) -> bool {
    let centre = spring::types::Float3 { x, y, z };
    spring::display::is_sphere_in_view(centre, magnitude.max(VIEW_RADIUS)).unwrap_or(true)
}

const VIEW_RADIUS: f32 = 150.0;

/// A value in [-1, 1) from the unsynced RNG: the scatter is only drawn, never
/// part of the game state.
fn signed_random() -> f32 {
    spring::random::unsynced::range(-1.0, 1.0)
}
