use crate::{db::Symbol, diagnostic::Span};

pub struct Token<'db> {
    pub kind: TokenKind<'db>,
    pub span: Span<'db>,
}

pub enum TokenKind<'db> {
    Ident(Symbol<'db>),

    /// `{`
    OpenBrace,
    /// `}`
    CloseBrace,
    /// `[`
    OpenBracket,
    /// `]`
    CloseBracket,
    /// `(`
    OpenParen,
    /// `)`
    CloseParen,

    /// An end of input.
    Eof,
}

impl<'db> Token<'db> {
    pub const fn new(kind: TokenKind<'db>, span: Span<'db>) -> Self {
        Self { kind, span }
    }
}
