use std::path::PathBuf;

#[salsa::tracked(debug)]
pub struct SourceFileId<'db> {
    #[tracked]
    pub value: usize,
}

#[salsa::tracked(debug)]
pub struct SourceFile<'db> {
    #[tracked]
    pub id: SourceFileId<'db>,
    #[tracked]
    #[returns(ref)]
    pub path: PathBuf,
    #[tracked]
    #[returns(ref)]
    pub contents: String,
}
