/// Parsed token.
/// It doesn't contain information about data that has been parsed,
/// only the type of the token and its size.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Token {
    pub kind: TokenKind,
    pub len: u32,
}

/// Enum representing common lexeme types.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TokenKind {
    /// Any whitespace character sequence.
    Whitespace,

    /// A line comment, e.g. `// comment`.
    LineComment,
    /// A block comment, e.g. `/* comment */`.
    BlockComment { terminated: bool },

    /// `=`
    Eq,
    /// `<`
    Lt,
    /// `>`
    Gt,
    /// `!`
    Bang,
    /// `~`
    Tilde,
    /// `+`
    Plus,
    /// `-`
    Minus,
    /// `*`
    Star,
    /// `/`
    Slash,
    /// `%`
    Percent,
    /// `^`
    Caret,
    /// `&`
    And,
    /// `|`
    Or,
    /// `@`
    At,
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
    /// `$`
    Dollar,
    /// `?`
    Question,
    /// `(`
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

    /// An identifier or keyword, e.g. `ident` or `prop`.
    Ident,

    /// A literal, e.g. `123` or `"hello"`.
    Literal { kind: LiteralKind },

    /// Unknown token, not expected by the lexer, e.g. "№".
    Unknown,

    /// End of input.
    Eof,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LiteralKind {
    Int,
    Float,
    Rune { terminated: bool },
    Str { terminated: bool },
}
