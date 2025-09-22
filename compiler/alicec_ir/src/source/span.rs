#[salsa::tracked(debug)]
pub struct Span<'db> {
    #[tracked]
    pub start: BytePos,
    #[tracked]
    pub end: BytePos,
}

#[derive(Debug)]
pub struct Spanned<'db, T> {
    pub value: T,
    pub span: Span<'db>,
}

impl<'db, T> Spanned<'db, T> {
    pub fn new(value: T, span: Span<'db>) -> Self {
        Self { value, span }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, salsa::Update)]
pub struct BytePos(usize);

impl From<usize> for BytePos {
    fn from(value: usize) -> Self {
        Self(value)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, salsa::Update)]
pub struct CharPos(usize);

impl From<usize> for CharPos {
    fn from(value: usize) -> Self {
        Self(value)
    }
}
