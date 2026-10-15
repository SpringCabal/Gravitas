use springcabal::music::MusicLoop;

pub fn music() -> MusicLoop {
    MusicLoop::new(TRACK, VOLUME)
}

const TRACK: &str = "sounds/music.ogg";
const VOLUME: f32 = 0.3;
