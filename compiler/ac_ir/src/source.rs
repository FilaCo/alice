#[salsa::tracked(debug)]
pub struct FileId<'db> {
    #[tracked]
    pub value: u32,
}
