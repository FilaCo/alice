use std::path::Path;

use salsa::{Database, Storage};

use crate::source::{SourceFileId, SourceMap};

#[salsa::db]
pub trait AcDbTrait: Database {}

#[derive(Clone)]
#[salsa::db]
pub struct AcDb {
    storage: Storage<Self>,
    sources: SourceMap,
}

impl AcDb {
    pub fn new() -> Self {
        Self {
            storage: Storage::default(),
            sources: SourceMap::new(),
        }
    }

    pub fn get_source_file(&self, id: SourceFileId) {
        todo!()
    }

    pub fn add_source_file(&self, fpath: &Path) {
        todo!()
    }
}

impl Database for AcDb {}

#[salsa::db]
impl AcDbTrait for AcDb {}
