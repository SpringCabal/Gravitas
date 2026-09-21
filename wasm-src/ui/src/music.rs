use crate::state;

pub fn update(delta_seconds: f32) {
    let Some(started) = state::with_state(|ui| ui.music_started) else {
        return;
    };
    if !started {
        start();
        return;
    }
    let played = state::with_state(|ui| {
        ui.music_time += delta_seconds;
        ui.music_time
    })
    .unwrap_or(0.0);
    let track_time = state::with_state(|ui| ui.music_track_time).unwrap_or(0.0);
    if track_time > 0.0 && played > track_time - BUFFER {
        start();
    }
}

const VOLUME: f32 = 0.3;
const TRACK: &str = "sounds/music.ogg";
const BUFFER: f32 = 0.015;

fn start() {
    if spring::sound::play_sound_stream(TRACK, VOLUME, false).is_err() {
        return;
    }
    let track_time = spring::sound::get_sound_stream_time(0).unwrap_or(0.0);
    let _ = state::with_state(|ui| {
        ui.music_started = true;
        ui.music_time = 0.0;
        ui.music_track_time = track_time;
    });
}
