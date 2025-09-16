/// Parsed token.
/// It doesn't contain information about data that has been parsed,
/// only the type of the token and its size.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Token {
    pub kind: TokenKind,
    pub len: usize,
}

/// Enum representing common lexeme types.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TokenKind {
    /// A line comment, e.g. `// comment`.
    LineComment,
    /// A block comment, e.g. `/* block comment */`.
    ///
    /// Block comments can be recursive, so a sequence like `/* /* */`
    /// will not be considered terminated and will result in a parsing error.
    BlockComment { is_terminated: Terminated },

    /// Any whitespace character sequence.
    Whitespace,

    /// An identifier or keyword, e.g. `ident` or `component`.
    Ident,

    /// Literals, e.g. `12u8`, `1.0e-40`, `"123"`. Note that `_` is an invalid
    /// suffix, but may be present here on string and float literals. Users of
    /// this type will need to check for and reject that case.
    ///
    /// See [LiteralKind] for more details.
    Literal {
        kind: LiteralKind,
        suffix_start: usize,
    },

    /// `;`
    Semi,
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

    /// An unknown token, not expected by the lexer, e.g. "№".
    Unknown,

    /// An end of input.
    Eof,
}

/// Enum representing the empty state of someting, e.g. block comment.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Terminated {
    No,
    Yes,
}

/// Enum representing the empty state of someting, e.g. empty exponent in a float literal.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Empty {
    No,
    Yes,
}

/// Enum representing the literal types supported by the lexer.
///
/// Note that the suffix is *not* considered when deciding the `LiteralKind` in
/// this type. This means that float literals like `1f32` are classified by this
/// type as `Int`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LiteralKind {
    /// `"abc"`, `"abc`
    String { is_terminated: Terminated },
    /// `12_u8`, `0o100`, `0b120i99`, `1f32`.
    Int { base: Base, empty_int: Empty },
    /// `12.34f32`, `1e3`, but not `1f32`.
    Float { base: Base, empty_exponent: Empty },
}

/// Base of numeric literal encoding according to its prefix.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Base {
    /// Literal starts with "0b" or "0B".
    Binary = 2,
    /// Literal starts with "0o" or "0O".
    Octal = 8,
    /// Literal doesn't contain a prefix.
    Decimal = 10,
    /// Literal starts with "0x" or "0X".
    Hexadecimal = 16,
}
