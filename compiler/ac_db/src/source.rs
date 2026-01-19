use std::{
    path::{Path, PathBuf},
    sync::{Arc, RwLock},
};

use memmap2::Mmap;

#[derive(Debug, Clone)]
pub(crate) struct SourceMap {
    inner: Arc<RwLock<Vec<SourceFile>>>,
}

impl SourceMap {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(RwLock::new(Vec::new())),
        }
    }

    pub fn add(&self, fpath: &Path) -> SourceFileId {
        // let mut storage = self.inner.write().expect("unable to write sources");
        // let fid = SourceFileId(storage.len());
        // let path = fpath.canonicalize().expect("unable to canonicalize path");
        // let file = File::open(path.clone()).expect("unable to open file");
        // let handle = unsafe { Mmap::map(&file).expect("unable to memmap file") };
        // sources.push(SourceFile::new(self, fid, path, Arc::new(handle)));
        // fid
        todo!()
    }

    pub fn get(&self, id: SourceFileId) {
        todo!()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SourceFileId(usize);

impl From<SourceFileId> for usize {
    fn from(id: SourceFileId) -> Self {
        id.0
    }
}

#[derive(Debug, Clone)]
pub struct SourceFile {
    id: SourceFileId,
}
