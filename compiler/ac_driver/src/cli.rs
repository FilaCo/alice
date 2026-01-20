use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Clone, Debug)]
#[command(version)]
pub struct Cli {
    /// Input source file
    pub input: PathBuf,
}
