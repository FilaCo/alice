use std::{
    fs::File,
    path::{Path, PathBuf},
    sync::{Arc, RwLock},
};

use memmap2::Mmap;
use salsa::{Database, Storage};

use crate::{
    cli::Cli,
    error::AcError,
    ir::source::{SourceFile, SourceFileId},
};

#[salsa::db]
pub trait AcDbTrait: Database {}

#[derive(Clone)]
#[salsa::db]
pub struct AcDb {
    storage: Storage<Self>,
    args: Cli,
    sources: Arc<RwLock<Vec<SourceFile>>>,
}

impl AcDb {
    pub fn new(args: Cli) -> Self {
        Self {
            storage: Storage::default(),
            args,
            sources: Arc::new(RwLock::new(Vec::new())),
        }
    }

    pub fn args(&self) -> &Cli {
        &self.args
    }

    pub fn add_source_file(&self, fpath: &Path) -> SourceFileId {
        let mut sources = self.sources.write().expect("unable to write sources");
        let fid = SourceFileId::new(self, sources.len());
        let path = fpath.canonicalize().expect("unable to canonicalize path");
        let file = File::open(path.clone()).expect("unable to open file");
        let handle = unsafe { Mmap::map(&file).expect("unable to memmap file") };
        sources.push(SourceFile::new(self, fid, path, Arc::new(handle)));
        fid
    }

    pub fn get_source_file(&self, fid: SourceFileId) -> Option<SourceFile> {
        let sources = self.sources.read().expect("unable to read sources");
        sources.get(fid.value(self)).copied()
    }
}

impl Database for AcDb {}

#[salsa::db]
impl AcDbTrait for AcDb {}
