use crate::source::FileId;

#[salsa::tracked(debug)]
pub struct Span<'db> {
    #[tracked]
    pub lo: u32,
    #[tracked]
    pub hi: u32,
    #[tracked]
    pub file: FileId<'db>,
}
