/// Health colours from empty to full, shared by the HUD and the healthbars.
const RAMP: [[f32; 3]; 3] = [[1.0, 0.0, 0.0], [0.8, 0.6, 0.0], [0.0, 0.75, 0.0]];

/// The colour for a health fraction in `[0, 1]`.
pub fn color(ratio: f32) -> [f32; 3] {
    let position = (RAMP.len() - 1) as f32 * ratio.clamp(0.0, 1.0);
    let index = (position as usize).min(RAMP.len() - 2);
    let blend = position - index as f32;
    let low = RAMP[index];
    let high = RAMP[index + 1];
    core::array::from_fn(|channel| low[channel] * (1.0 - blend) + high[channel] * blend)
}
