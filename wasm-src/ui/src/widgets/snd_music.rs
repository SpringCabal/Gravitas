use spring_addons::AddonContext;
use spring_addons::ui::Widget;

use crate::music::Music;
use crate::state::Global;

#[derive(Default)]
pub struct SndMusicWidget {
    music: Music,
}

impl Widget<Global> for SndMusicWidget {
    fn name(&self) -> &'static str {
        "SndMusic"
    }

    fn update(&self, _ctx: &AddonContext<'_, Global>, dt: f32) {
        self.music.update(dt.max(0.0));
    }
}
