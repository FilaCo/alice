use salsa::Database;

use crate::{source::Span, syntax::Symbol};

#[salsa::tracked(debug)]
pub struct Token<'db> {
    pub kind: TokenKind<'db>,
    pub span: Span<'db>,
}

impl<'db> Token<'db> {
    pub fn dummy(db: &'db dyn Database) -> Self {
        Token::new(db, TokenKind::Quest, Span::dummy(db))
    }

    pub fn eof(db: &'db dyn Database, span: Span<'db>) -> Self {
        Token::new(db, TokenKind::Eof, span)
    }

    pub fn glue(&self, other: &Token<'db>, db: &'db dyn Database) -> Option<Token<'db>> {
        None
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, salsa::Update)]
pub enum TokenKind<'db> {
    /// `NL = LF | (CR [LF])`
    NL,
    /// `=`
    Eq,
    /// `<`
    Lt,
    /// `>`
    Gt,
    /// `!`
    Excl,
    /// `+`
    Plus,
    /// `-`
    Minus,
    /// `*`
    Star,
    /// `/`
    Slash,
    /// `.`
    Dot,
    /// `,`
    Comma,
    /// `;`
    Semi,
    /// `:`
    Colon,
    /// `#`
    Hash,
    /// `?`
    Quest,
    /// `(``
    LParen,
    /// `)`
    RParen,
    /// `{`
    LBrace,
    /// `}`
    RBrace,
    /// `[`
    LBracket,
    /// `]`
    RBracket,

    /// `==`
    EqEq,
    /// `!=`
    Ne,
    /// `<=`
    Le,
    /// `>=`
    Ge,
    /// `::`
    ColonColon,

    /// An identifier or keyword, e.g. `ident` or `prop`.
    Ident { sym: Symbol<'db> },

    /// A literal, e.g. `123` or `"hello"`.
    Literal { kind: LiteralKind, sym: Symbol<'db> },

    /// Unknown token, not expected by the lexer, e.g. "№".
    Unknown,

    /// End of input.
    Eof,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, salsa::Update)]
pub enum LiteralKind {
    Int { base: Base },
    Float { base: Base },
    Rune { terminated: bool },
    Str { terminated: bool },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, salsa::Update)]
pub enum Base {
    Bin = 2,
    Oct = 8,
    Dec = 10,
    Hex = 16,
}
