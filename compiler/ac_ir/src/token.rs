use crate::{span::Span, value::Symbol};

#[salsa::tracked(debug)]
pub struct Token<'db> {
    #[tracked]
    pub kind: TokenKind,
    #[tracked]
    pub symbol: Symbol<'db>,
    #[tracked]
    pub span: Span<'db>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, salsa::Update)]
pub enum TokenKind {
    /// A block comment, e.g. `/* block comment */`.
    ///
    /// Block comments can be recursive, so a sequence like `/* /* */`
    /// will not be considered terminated and will result in a parsing error.
    BlockComment {
        terminated: bool,
    },

    /// An identifier or keyword, e.g. `ident` or `prop`.
    Ident,

    Literal {
        kind: LiteralKind,
    },

    /// `;`
    Semi,
    /// `,`
    Comma,
    /// `.`
    Dot,
    /// `{`
    LBrace,
    /// `}`
    RBrace,
    /// `[`
    LBracket,
    /// `]`
    RBracket,
    /// `(``
    LParen,
    /// `)`
    RParen,
    /// `:`
    Colon,
    /// `=`
    Eq,
    /// `!`
    Ex,
    /// `<`
    Lt,
    /// `>`
    Gt,
    /// `-`
    Minus,
    /// `+`
    Plus,
    /// `*`
    Star,
    /// `/`
    Slash,

    /// `==`
    EqEq,
    /// `!=`
    Ne,
    /// `<=`
    Le,
    /// `>=`
    Ge,

    /// Unknown token, not expected by the lexer, e.g. "№"
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, salsa::Update)]
pub enum LiteralKind {
    Int { base: Base },
    Float { base: Base },
    Rune { terminated: bool },
    Str { terminated: bool },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, salsa::Update)]
pub enum Base {
    Bin = 2,
    Oct = 8,
    Dec = 10,
    Hex = 16,
}
