use alloc::vec::Vec;

const BAR_HEIGHT: f32 = 2.3;
const BAR_ALPHA: f32 = 0.85;
const BAR_VALUE_ALPHA: f32 = 0.9;

pub struct Bar {
    pub progress: f32,
    pub color: [f32; 4],
}

pub struct BarStack {
    bars: Vec<Bar>,
}

impl BarStack {
    pub fn new() -> Self {
        Self { bars: Vec::new() }
    }

    pub fn push(&mut self, progress: f32, color: [f32; 4]) {
        self.bars.push(Bar {
            progress: progress.clamp(0.0, 1.0),
            color,
        });
    }

    pub fn is_empty(&self) -> bool {
        self.bars.is_empty()
    }

    pub fn iter(&self) -> impl Iterator<Item = (f32, &Bar)> {
        self.bars.iter().enumerate().map(|(index, bar)| {
            (
                BAR_BASELINE - (index as f32) * (BAR_HEIGHT + BAR_SPACING),
                bar,
            )
        })
    }
}

pub const EMP: [f32; 4] = [0.50, 0.50, 1.00, BAR_ALPHA];
pub const EMP_PAUSED: [f32; 4] = [0.40, 0.40, 0.80, BAR_ALPHA];
pub const EMP_BLINK: [f32; 4] = [0.60, 0.60, 0.90, BAR_ALPHA];
pub const CAPTURE: [f32; 4] = [1.00, 0.50, 0.00, BAR_ALPHA];
pub const BUILD: [f32; 4] = [0.75, 0.75, 0.75, BAR_ALPHA];
pub const RELOAD: [f32; 4] = [0.00, 0.60, 0.60, BAR_ALPHA];

const BAR_SPACING: f32 = 1.0;
// Lua's healthbar widget starts its first bar at y=20 relative to the
// unit-radius anchor. Starting at zero puts the bar through the model.
const BAR_BASELINE: f32 = 20.0;
const HEALTH_COLORS: [[f32; 4]; 3] = [
    [1.0, 0.0, 0.0, BAR_VALUE_ALPHA],
    [0.8, 0.60, 0.0, BAR_VALUE_ALPHA],
    [0.0, 0.75, 0.0, BAR_VALUE_ALPHA],
];

pub fn health_color(slider: f32) -> [f32; 4] {
    let count = HEALTH_COLORS.len();
    if slider >= 1.0 {
        return HEALTH_COLORS[count - 1];
    }
    let slider = slider.clamp(0.0, 1.0);
    let position = 1.0 + (count as f32 - 1.0) * slider;
    let index = (position.floor() as usize).clamp(1, count - 1);
    let blend = position - index as f32;
    let inverse = 1.0 - blend;
    let low = HEALTH_COLORS[index - 1];
    let high = HEALTH_COLORS[index];
    [
        low[0] * inverse + high[0] * blend,
        low[1] * inverse + high[1] * blend,
        low[2] * inverse + high[2] * blend,
        low[3] * inverse + high[3] * blend,
    ]
}
