use std::path::PathBuf;

#[derive(Debug)]
pub struct AlicecConfig {
    pub input: PathBuf,
}

impl AlicecConfig {
    pub const fn new(input: PathBuf) -> Self {
        Self { input }
    }
}
