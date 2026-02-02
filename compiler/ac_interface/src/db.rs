use std::path::Path;

use ac_db::db::AcDbTrait;
use ac_ir::input::SourceFile;
use salsa::Storage;

use crate::compiler::Config;

#[salsa::db]
#[derive(Clone)]
pub struct AcDb {
    storage: Storage<Self>,
    cfg: Config,
}

impl From<Config> for AcDb {
    fn from(cfg: Config) -> Self {
        Self {
            storage: Storage::default(),
            cfg,
        }
    }
}

#[salsa::db]
impl AcDbTrait for AcDb {
    fn input(&self) -> SourceFile {
        todo!()
    }
}

impl salsa::Database for AcDb {}
