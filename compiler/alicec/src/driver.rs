use crate::cli::Cli;
use alicec_db::prelude::AlicecDb;
use clap::Parser;

pub struct Driver {
    db: AlicecDb,
}

impl Driver {
    pub fn new() -> Self {
        let args = Cli::parse();

        let db = AlicecDb::default();
        Self { db }
    }

    pub fn run(&mut self) {}
}

impl Default for Driver {
    fn default() -> Self {
        Self::new()
    }
}
