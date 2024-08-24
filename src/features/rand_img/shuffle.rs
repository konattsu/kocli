use super::file_rename::FileRename;
use std::{fs, io, path};

use crate::features::utils::config_loader::ConfigLoader;
use serde::{de::DeserializeOwned, Deserialize};

#[derive(Deserialize)]
struct RandImg {
    directory: String,
}
#[derive(Deserialize)]
struct Config {
    rand_img: RandImg,
}
impl ConfigLoader for Config {}

pub fn run(dirname: Option<String>) {
    let dirname = match decide_dirname::<Config>(dirname) {
        Ok(dir) => dir,
        Err(e) => {
            eprintln!("Error accessing setting files: {}", e);
            return;
        }
    };

    let read_dir = match fs::read_dir(dirname) {
        Ok(read_dir) => read_dir,
        Err(e) => {
            eprintln!("Error unable to access directory: {}", e);
            return;
        }
    };
    match FileRename::new(read_dir) {
        Ok(f) => {
            if let Err(e) = f.rename_files() {
                eprintln!("Error renaming files: {}", e);
            } else {
                println!("Finished shuffling files")
            };
        }
        Err(e) => eprintln!("Error accessing files: {}", e),
    };
}

fn decide_dirname<T>(dirname: Option<String>) -> io::Result<path::PathBuf>
where
    T: ConfigLoader + DeserializeOwned,
{
    if let Some(dir) = dirname {
        return Ok(path::PathBuf::from(dir));
    }

    let config = T::load_config::<Config>()?;

    let dirname = path::PathBuf::from(config.rand_img.directory);
    if dirname.is_dir() {
        return Ok(dirname);
    }
    Err(io::Error::new(
        io::ErrorKind::InvalidData,
        "The path specified in settings.toml is not a file",
    ))
}
