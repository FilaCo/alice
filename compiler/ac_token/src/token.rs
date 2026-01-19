use ac_db::span::Span;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Token<'db> {
    pub kind: TokenKind<'db>,
    pub span: Span<'db>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TokenKind<'db> {
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
    /// `(`
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
    /// `%`
    Percent,

    /// An end of input.
    Eof,
}
