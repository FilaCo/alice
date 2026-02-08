#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum TokenType {
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
    Ident,

    /// A literal, e.g. `123` or `"hello"`.
    Literal,

    /// End of input.
    Eof,
}
