use rand::{seq::SliceRandom, thread_rng};
use std::{ffi::OsStr, fs, io, path::PathBuf};

use crate::features::utils::config_loader::ConfigLoader;
use serde::Deserialize;

#[derive(Deserialize)]
struct Remind {
    directory: String,
}
#[derive(Deserialize)]
struct Config {
    remind: Remind,
}
impl ConfigLoader for Config {}

pub enum Audio {
    File(PathBuf),
    Folder(PathBuf),
    Beep,
}
impl Audio {
    pub fn new(path: Option<String>) -> Audio {
        let decide_audio_type = {
            |path: PathBuf| {
                if path.is_file() {
                    Audio::File(path)
                } else if path.is_dir() {
                    Audio::Folder(path)
                } else {
                    Audio::Beep
                }
            }
        };

        // TODO ここのmatch見やすく
        match path {
            Some(path) => {
                let path = PathBuf::from(path);
                decide_audio_type(path)
            }
            None => {
                let path = Self::load_config();
                match path {
                    Ok(p) => decide_audio_type(PathBuf::from(p)),
                    Err(_) => Audio::Beep,
                }
            }
        }
    }

    fn load_config() -> io::Result<String> {
        let config = Config::load_config::<Config>()?;
        Ok(config.remind.directory)
    }

    pub fn select_audio_file(&self) -> io::Result<PathBuf> {
        match self {
            Audio::File(file) => Ok(file.to_path_buf()),
            Audio::Folder(folder) => {
                let mut valid_files =
                    self.retrieve_audio_file(fs::read_dir(folder)?)?;
                let mut rng = thread_rng();
                valid_files.shuffle(&mut rng);
                Ok(valid_files[0].clone())
            }
            Audio::Beep => Err(io::Error::new(
                io::ErrorKind::Other,
                "Beep does not support file selection",
            )),
        }
    }

    fn retrieve_audio_file(&self, read_dir: fs::ReadDir) -> io::Result<Vec<PathBuf>> {
        let valid_ext = String::from("mp3");
        let mut valid_files: Vec<PathBuf> = Vec::new();
        for entry in read_dir {
            let path = match entry {
                Ok(p) => p.path(),
                Err(_) => continue,
            };
            if let Some(ext) = path.extension() {
                if ext == OsStr::new(&valid_ext) {
                    valid_files.push(path);
                }
            }
        }
        if valid_files.is_empty() {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                "No valid files found in folder",
            ));
        }
        Ok(valid_files)
    }
}
