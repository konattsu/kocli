use kocli::features::remind::{play_sound, Audio, TimeMode};

fn main() {
    let playback_time: u64 = 12;
    let audio_file = Audio::new(None);
    let delay_secs = TimeMode::calc_time(&TimeMode::Time, "20:00:00").unwrap();

    play_sound::run(audio_file, playback_time, delay_secs);
}
