use std::path::{Path, PathBuf};

use salsa::{Database, Storage};

use crate::{cli::Cli, error::AlicecError, ir::source::SourceFile};

mod source_map;

#[salsa::db]
pub trait AlicecDbTrait: Database {}

#[derive(Clone)]
#[salsa::db]
pub struct AlicecDb {
    storage: Storage<Self>,
    args: Cli,
}

impl AlicecDb {
    pub fn new(args: Cli) -> Self {
        Self {
            storage: Storage::default(),
            args,
        }
    }

    pub fn args(&self) -> &Cli {
        &self.args
    }

    pub fn add_source_file(&self, fpath: &Path) -> Result<SourceFile, AlicecError> {
        todo!()
    }
}

impl Database for AlicecDb {}

#[salsa::db]
impl AlicecDbTrait for AlicecDb {}
