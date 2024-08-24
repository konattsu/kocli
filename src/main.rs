use clap::{Args, Parser, Subcommand};

use kocli::features::{
    convert_path::convert,
    rand_img::shuffle,
    remind::{play_sound, Audio, TimeMode},
};

#[derive(Parser)]
#[command(version, about)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Plays a notification sound
    Remind(RemindArgs),
    /// Slash as delimiter of copied paths from explorer
    Path,
    /// Shuffle images in folder
    RandImg(RandImgArgs),
}

#[derive(Args)]
struct RemindArgs {
    /// Select `time, A specific time` or `later, How long from now`
    mode: TimeMode, /* remind/time_mode.rs */
    /// Time to wait before playing sound
    #[arg(short, long)]
    delay: String,
    /// Time to play sound
    #[arg(short, long, default_value_t = 15)]
    playback_secs: u64,
    /// Path to the audio file or folder
    #[arg(short, long, value_name = "FILE/FOLDER")]
    audio: Option<String>,
}

#[derive(Args)]
struct RandImgArgs {
    /// Folder with images to shuffle
    #[arg(short, long)]
    folder: Option<String>,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Remind(remind_args) => {
            let audio_file = remind_args.audio;
            let playback_secs = remind_args.playback_secs;
            let delay_secs = match remind_args.mode.calc_time(&remind_args.delay) {
                Ok(secs) => secs,
                Err(e) => {
                    eprintln!("Error calculating the time: {}", e);
                    return;
                }
            };
            play_sound::run(Audio::new(audio_file), playback_secs, delay_secs);
        }
        Commands::Path => {
            convert::run();
        }
        Commands::RandImg(rand_img_args) => {
            shuffle::run(rand_img_args.folder);
        }
    }
}
