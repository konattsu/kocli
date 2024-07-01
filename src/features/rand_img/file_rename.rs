use std::{
    fs, io,
    iter::zip,
    path::{self, PathBuf},
};

use rand::{seq::SliceRandom, thread_rng};

const ALLOWED_EXT: &str = "png";

pub struct FileRename {
    original: Vec<path::PathBuf>,
    shuffled: Vec<path::PathBuf>,
    dir_name: path::PathBuf,
}
impl FileRename {
    pub fn new(read_dir: fs::ReadDir) -> io::Result<FileRename> {
        let original: Vec<path::PathBuf> = read_dir
            .filter_map(|entry| entry.ok())
            .map(|file| file.path())
            .filter(|path| path.extension().map_or(false, |ext| ext == ALLOWED_EXT))
            .collect();
        if original.is_empty() {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                "No valid files found",
            ));
        }

        let shuffled = FileRename::shuffle_names(original.clone());
        let dir_name = original[0]
            .parent()
            .ok_or_else(|| {
                io::Error::new(
                    io::ErrorKind::NotFound,
                    "Directory name could not be obtained",
                )
            })?
            .to_path_buf();

        Ok(FileRename {
            original,
            shuffled,
            dir_name,
        })
    }

    fn shuffle_names(mut path_buf: Vec<path::PathBuf>) -> Vec<path::PathBuf> {
        let mut rng = thread_rng();
        path_buf.shuffle(&mut rng);
        path_buf
    }

    pub fn rename_files(&self) -> Result<(), io::Error> {
        const TMP_STR: &str = "PreventDuplicateNames";
        let mut filenames_tmp: Vec<PathBuf> = Vec::new();

        for src in &self.original {
            let file_stem = src.file_stem().unwrap().to_str().ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidData, "Invalid file name")
            })?;
            let mut filename_tmp = self.dir_name.clone();
            filename_tmp.push(format!("{}{}", file_stem, TMP_STR));
            filename_tmp.set_extension(ALLOWED_EXT);
            filenames_tmp.push(filename_tmp.clone());
            fs::rename(src, &filename_tmp)?;
        }

        for (tmp, dst) in zip(filenames_tmp, &self.shuffled) {
            fs::rename(tmp, dst)?;
        }
        Ok(())
    }
}
