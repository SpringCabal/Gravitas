use springcabal::music::MusicLoop;

/// `snd_music.lua`: the one track, looped.
pub fn music() -> MusicLoop {
    MusicLoop::new(TRACK, VOLUME)
}

const TRACK: &str = "sounds/music.ogg";
const VOLUME: f32 = 0.3;
