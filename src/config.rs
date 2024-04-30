use crate::{
    prelude::{Path, PathBuf, INPUT_DIR},
    Error, Result,
};

pub struct Config {
    pub file_loc: PathBuf,
    pub cur_dir: PathBuf,
}

impl Config {
    pub fn new() -> Result<Self> {
        let cur_dir = std::env::current_exe()?.parent().unwrap().to_path_buf();
        let data_folder = cur_dir.join(INPUT_DIR);

        match Self::check_input_dir(data_folder.clone()) {
            Ok(_) => (),
            Err(e) => return Err(e),
        }

        let file_loc = match Self::check_input_file(data_folder.join("input.json")) {
            Ok(file_loc) => file_loc,
            Err(e) => return Err(e),
        };

        Ok(Config { cur_dir, file_loc })
    }

    pub fn check_input_dir<P: AsRef<Path>>(input_dir: P) -> Result<PathBuf> {
        match !input_dir.as_ref().exists() && input_dir.as_ref().is_dir() {
            true => {
                std::fs::create_dir_all(input_dir.as_ref())?;
                eprintln!(
                    "Input directory created: {:?}, exiting...",
                    input_dir.as_ref()
                );
                Err(Error::InputDir(input_dir.as_ref().to_path_buf()))
            }
            false => match input_dir.as_ref().is_dir() {
                true => Ok(input_dir.as_ref().to_path_buf()),
                false => {
                    std::fs::create_dir_all(input_dir.as_ref())?;
                    eprintln!(
                        "Input directory not found: {:?}, exiting...",
                        input_dir.as_ref()
                    );
                    return Err(Error::InputDir(input_dir.as_ref().to_path_buf()));
                }
            },
        }
    }

    pub fn check_input_file<P: AsRef<Path>>(input_file: P) -> Result<PathBuf> {
        match !input_file.as_ref().exists() && input_file.as_ref().is_file() {
            true => {
                eprintln!(
                    "Input file not found: {:?}, exiting...",
                    input_file.as_ref()
                );
                Err(Error::InputFile(input_file.as_ref().to_path_buf()))
            }
            false => match input_file.as_ref().is_file() {
                true => Ok(input_file.as_ref().to_path_buf()),
                false => {
                    eprintln!(
                        "Input file not found: {:?}, exiting...",
                        input_file.as_ref()
                    );
                    return Err(Error::InputFile(input_file.as_ref().to_path_buf()));
                }
            },
        }
    }
}
