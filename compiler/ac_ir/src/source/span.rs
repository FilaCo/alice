use salsa::Database;

#[salsa::tracked(debug)]
pub struct Span<'db> {
    #[tracked]
    pub start: usize,
    #[tracked]
    pub end: usize,
}

impl<'db> Span<'db> {
    pub fn dummy(db: &'db dyn Database) -> Self {
        Span::new(db, 0, 0)
    }
}
