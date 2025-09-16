use alicec_db::prelude::Symbol;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Token<'db> {
    Ident(Symbol<'db>),

    Literal(LiteralData<'db>),

    /// `,`
    Comma,
    /// `.`
    Dot,
    /// `:`
    Colon,
    /// `~`
    Tilde,
    /// `?`
    Quest,
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
    /// `&`
    Amp,
    /// `|`
    Pipe,
    /// `+`
    Plus,
    /// `/`
    Slash,
    /// `*`
    Star,
    /// `%`
    Percent,

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

    /// `..`
    DotDot,
    /// `==`
    EqEq,
    /// `=>`
    EqGt,

    /// An end of input.
    Eof,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LiteralData<'db> {
    pub symbol: Symbol<'db>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LiteralKind {}
