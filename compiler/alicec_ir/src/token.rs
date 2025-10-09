use crate::{source::Span, symbol::Symbol};
use TokenKind::*;
use alicec_db::prelude::AlicecDbTrait;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Token<'db> {
    pub kind: TokenKind<'db>,
    pub span: Span<'db>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TokenKind<'db> {
    Lit(Lit<'db>),

    /// `-`
    Minus,
    /// `+`
    Plus,
    /// `/`
    Slash,
    /// `*`
    Star,

    /// `(`
    LParen,
    /// `)`
    RParen,

    /// Dummy for parser needs.
    Dummy,

    /// An end of input.
    Eof,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, salsa::Update)]
pub struct Lit<'db> {
    pub kind: LitKind,
    pub symbol: Symbol<'db>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LitKind {
    Int,
    Float,
    Err,
}

impl<'db> Token<'db> {
    pub fn new(kind: TokenKind<'db>, span: Span<'db>) -> Self {
        Self { kind, span }
    }

    pub fn dummy(db: &'db dyn AlicecDbTrait) -> Self {
        Self::new(Dummy, Span::dummy(db))
    }
}

impl<'db> PartialEq<TokenKind<'db>> for Token<'db> {
    fn eq(&self, other: &TokenKind<'db>) -> bool {
        self.kind == *other
    }
}
