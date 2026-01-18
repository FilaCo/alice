use std::{path::PathBuf, sync::Arc};

use memmap2::Mmap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SourceFileId(usize);

#[salsa::input]
pub struct SourceFile {
    pub id: SourceFileId,
    contents: Arc<Mmap>,
}
