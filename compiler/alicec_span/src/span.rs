#[salsa::tracked(debug)]
pub struct Span<'db> {
    #[tracked]
    pub start: usize,
    #[tracked]
    pub end: usize,
}

#[derive(Debug)]
pub struct Spanned<'db, T> {
    pub span: Span<'db>,
    pub value: T,
}
