use crate::source::BytePos;
use alicec_db::prelude::AlicecDbTrait;

#[salsa::tracked(debug)]
pub struct Span<'db> {
    #[tracked]
    pub lo: BytePos,
    #[tracked]
    pub hi: BytePos,
}

impl<'db> Span<'db> {
    pub fn dummy(db: &'db dyn AlicecDbTrait) -> Self {
        Self::new(db, 0.into(), 0.into())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, salsa::Update)]
pub struct Spanned<'db, T: PartialEq + salsa::Update> {
    pub value: T,
    pub span: Span<'db>,
}

impl<'db, T: PartialEq + salsa::Update> Spanned<'db, T> {
    pub fn new(value: T, span: Span<'db>) -> Self {
        Self { value, span }
    }
}
