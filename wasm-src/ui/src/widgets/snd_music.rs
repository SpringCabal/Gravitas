use spring_addons::ui::Widget;

use crate::music;
use crate::state::Global;

#[derive(Default)]
pub struct SndMusicWidget;

impl SndMusicWidget {
    pub fn new() -> Self {
        Self
    }
}

impl Widget<Global> for SndMusicWidget {
    fn name(&self) -> &'static str {
        "SndMusic"
    }

    fn update(&self, _ctx: &spring_addons::AddonContext<'_, Global>, dt: f32) {
        music::update(dt.max(0.0));
    }
}
