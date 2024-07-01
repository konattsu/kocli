use std::{
    fs::{self, File},
    io::BufReader,
    path::PathBuf,
    thread,
    time::Duration,
};

use rodio::{source::SineWave, Decoder, OutputStream, Sink, Source};

use super::audio::Audio;

// TODO 非同期処理を組み込む
// マウスの動きを感知して止めたり

pub fn run(audio_file: Audio, playback_secs: u64, delay_secs: u64) {
    let selected_file: Option<PathBuf> = if let Audio::Beep = audio_file {
        None
    } else {
        match audio_file.select_audio_file() {
            Ok(file) => Some(file),
            Err(_) => None,
        }
    };

    thread::sleep(Duration::from_secs(delay_secs));
    play_sound(selected_file, playback_secs);
}

fn play_sound(audio_file: Option<PathBuf>, playback_secs: u64) {
    let (_stream, stream_handle) = match OutputStream::try_default() {
        Ok(handle) => handle,
        Err(e) => {
            eprintln!("Error initializing output stream: {}", e);
            return;
        }
    };

    let play_beep = || {
        let sink = match Sink::try_new(&stream_handle) {
            Ok(sink) => sink,
            Err(e) => {
                eprintln!("Error creating sink: {}", e);
                return;
            }
        };
        let source =
            SineWave::new(528.0).take_duration(Duration::from_secs(playback_secs));
        sink.append(source);
        sink.sleep_until_end();
    };
    let play_audio_file = |source: Decoder<BufReader<File>>| {
        let _ = stream_handle.play_raw(source.repeat_infinite().convert_samples());
        thread::sleep(Duration::from_secs(playback_secs));
    };

    match audio_file {
        Some(file) => {
            let file = match fs::File::open(file) {
                Ok(file) => BufReader::new(file),
                Err(e) => {
                    eprintln!("Error opening audio file: {}", e);
                    play_beep();
                    return;
                }
            };
            match Decoder::new(file) {
                Ok(source) => play_audio_file(source),
                Err(_) => {
                    eprintln!("Beeps due to the file could not be decoded");
                    play_beep();
                }
            }
        }
        None => play_beep(),
    }
}
