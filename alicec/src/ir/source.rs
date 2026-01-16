#[salsa::tracked(debug)]
pub struct SourceFileId<'db> {
    #[tracked]
    pub value: usize,
}
