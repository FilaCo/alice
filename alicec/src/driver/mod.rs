use clap::Parser;

use crate::driver::cli::Cli;

mod cli;

#[derive(Debug)]
pub struct Driver;

impl Driver {
    pub fn compile() {
        let args = Cli::parse();
    }
}
