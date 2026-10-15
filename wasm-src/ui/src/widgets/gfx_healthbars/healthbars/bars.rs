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
const BAR_BASELINE: f32 = 20.0;
pub fn health_color(fraction: f32) -> [f32; 4] {
    let [r, g, b] = crate::health::color(fraction);
    [r, g, b, BAR_VALUE_ALPHA]
}
