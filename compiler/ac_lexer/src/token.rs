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
    /* Hidden lexical rules */
    /// ```ebnf
    /// Whitespace = <SPACE U+0020> | <TAB U+0009> | <Form Feed U+000C>
    /// ```
    Whitespace,
    /// ```ebnf
    /// BlockComment = "/*" { BlockComment | <any character> } "*/"
    /// ```
    BlockComment { terminated: bool },
    /// ```ebnf
    /// LineComment  = "//" { <any character except NL> }
    /// ```
    LineComment,

    /// ```ebnf
    /// NewLine = <Line Feed U+000A> | (<Carriage Return U+000D> [<Line Feed U+000A>>])
    /// ```
    NewLine,

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
