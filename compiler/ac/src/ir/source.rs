use std::{path::PathBuf, sync::Arc};

use memmap2::Mmap;

#[salsa::input(debug)]
pub struct SourceFileId {
    pub value: usize,
}

#[salsa::input(debug)]
pub struct SourceFile {
    pub id: SourceFileId,
    #[returns(ref)]
    pub path: PathBuf,
    pub handle: Arc<Mmap>,
}
