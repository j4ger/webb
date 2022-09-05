use crate::log_expect::LogExpect;
use log::{error, info, warn};
use std::env;
use std::fs;
use std::io::prelude::*;
use std::path::PathBuf;
use std::process::abort;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct FileSource {
    file: PathBuf,
}

impl FileSource {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        let current_directory = env::current_dir().log_expect("Failed to get current directory.");
        let path: PathBuf = current_directory.join(path.into());
        info!("Saving to {}.", path.to_string_lossy());
        if path.is_dir() {
            error!("Error: {} is a directory.", path.to_string_lossy());
            abort();
        }
        FileSource { file: path }
    }

    pub fn store(&self, content: &impl serde::Serialize) {
        let file = self.file.as_path();
        if file.exists() {
            info!("Making backup file for {}.", file.to_string_lossy());
            let time_start = SystemTime::now();
            let since_the_epoch = time_start
                .duration_since(UNIX_EPOCH)
                .log_expect("Error: Time went backwards. How?");
            let new_filename = format!(
                "{}-{}",
                since_the_epoch.as_secs(),
                self.file
                    .file_name()
                    .log_expect("Error: File {} does not have a filename.",)
                    .to_string_lossy()
            );
            fs::rename(file, new_filename.as_str()).log_expect(&format!(
                "Error: Failed to create new file {}.",
                new_filename
            ));
            info!("Backup created: {}", new_filename);
        }
        let mut open_file = fs::File::create(file).log_expect(&format!(
            "Error: Failed to create file {}.",
            file.to_string_lossy()
        ));
        open_file
            .write_all(
                serde_yaml::to_string(&content)
                    .log_expect("Error: Failed to serialize data.")
                    .as_bytes(),
            )
            .log_expect(&format!(
                "Error: Failed to write to file {}.",
                file.to_string_lossy()
            ));
        info!("File {} saved.", file.to_string_lossy());
    }

    pub fn load<'a, T>(&self) -> Option<T>
    where
        T: serde::de::DeserializeOwned,
    {
        let file = self.file.as_path();
        info!("Loading file {}.", file.to_string_lossy());
        if file.exists() {
            let open_file = fs::File::open(file)
                .log_expect(&format!("Failed to open file {}.", file.to_string_lossy()));
            match serde_yaml::from_reader(open_file) {
                Ok(inner) => Some(inner),
                Err(error) => {
                    warn!("Failed to load file {}: {}.", file.to_string_lossy(), error);
                    None
                }
            }
        } else {
            warn!("File {} does not exist.", file.to_string_lossy());
            None
        }
    }
}
