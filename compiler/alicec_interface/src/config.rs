use std::path::PathBuf;

#[derive(Debug)]
pub struct Config {
    pub input: Vec<PathBuf>,
    pub include_dirs: Vec<PathBuf>,
}
