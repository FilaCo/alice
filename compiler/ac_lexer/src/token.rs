use TokenKind::*;

/// Parsed token.
/// It doesn't contain information about data that has been parsed,
/// only the type of the token and its size.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Token {
    pub kind: TokenKind,
    pub len: u32,
}

impl Token {
    pub(crate) fn new(kind: TokenKind, len: u32) -> Self {
        Self { kind, len }
    }

    pub(crate) fn eof() -> Self {
        Self { kind: Eof, len: 0 }
    }
}

/// Enum representing common lexeme types.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TokenKind {
    /// Any whitespace character sequence.
    Whitespace,

    /// A block comment, e.g. `/* comment */`.
    BlockComment,

    /// A line comment, e.g. `// comment`.
    LineComment,

    /// An identifier or keyword, e.g. `ident` or `prop`.
    Ident,

    Literal(LiteralKind),

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

    /// An unknown token, not expected by the lexer, e.g. "№".
    Unknown,

    /// An end of input.
    Eof,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LiteralKind {
    Int,
    Float,
}
