use core::cell::Cell;

const VOLUME: f32 = 0.3;
const TRACK: &str = "sounds/music.ogg";
/// Restart the track this long before it ends, so the loop has no gap.
const BUFFER: f32 = 0.015;

/// The music track, looped.
#[derive(Default)]
pub struct Music {
    started: Cell<bool>,
    played: Cell<f32>,
    track_time: Cell<f32>,
}

impl Music {
    pub fn update(&self, delta_seconds: f32) {
        if !self.started.get() {
            self.start();
            return;
        }
        let played = self.played.get() + delta_seconds;
        self.played.set(played);
        let track_time = self.track_time.get();
        if track_time > 0.0 && played > track_time - BUFFER {
            self.start();
        }
    }

    fn start(&self) {
        if spring::sound::play_sound_stream(TRACK, VOLUME, false).is_err() {
            return;
        }
        self.started.set(true);
        self.played.set(0.0);
        self.track_time
            .set(spring::sound::get_sound_stream_time(0).unwrap_or(0.0));
    }
}
