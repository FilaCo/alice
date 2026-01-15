#[salsa::tracked(debug)]
pub struct Span<'db> {
    #[tracked]
    pub lo: usize,
    #[tracked]
    pub hi: usize,
}
