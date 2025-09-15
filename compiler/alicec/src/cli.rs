use alicec_interface::config::AlicecConfig;
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(version, about, long_about)]
pub struct AlicecCli {
    input: PathBuf,
}

impl From<AlicecCli> for AlicecConfig {
    fn from(value: AlicecCli) -> Self {
        Self::new(value.input)
    }
}
