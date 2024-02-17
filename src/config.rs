use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub enum InputFile {
    Src(PathBuf),
    SingleFile(PathBuf),
}

impl InputFile {
    pub fn get(&self) -> &Path {
        match self {
            Self::Src(p) => p,
            Self::SingleFile(p) => p,
        }
    }
}

pub struct Config {
    pub input_file: InputFile,
    pub json: bool,
}
