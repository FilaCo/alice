use crate::source::FileId;

#[salsa::tracked(debug)]
pub struct Span<'db> {
    #[tracked]
    pub lo: usize,
    #[tracked]
    pub hi: usize,
    #[tracked]
    pub file: FileId<'db>,
}
