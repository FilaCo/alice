#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Token {
    pub kind: TokenKind,
    pub len: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
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

    /// Literals.
    Literal {
        kind: LiteralKind,
        suffix_start: usize,
    },

    /// `,`
    Comma,
    /// `.`
    Dot,
    /// `=`
    Eq,
    /// `<`
    Lt,
    /// `>`
    Gt,
    /// `-`
    Minus,
    /// `+`
    Plus,
    /// `/`
    Slash,
    /// `*`
    Star,
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

impl Token {
    pub const fn new(kind: TokenKind, len: usize) -> Self {
        Self { kind, len }
    }

    pub const fn eof() -> Self {
        Self::new(TokenKind::Eof, 0)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Terminated {
    No,
    Yes,
}

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
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum LiteralKind {
    String { is_terminated: Terminated },
    Int { base: Base, empty_int: Empty },
    Float { base: Base, empty_exp: Empty },
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn token_eof() {
        // arrange
        // act
        let actual = Token::eof();

        // assert
        assert_eq!(TokenKind::Eof, actual.kind);
        assert_eq!(0, actual.len)
    }

    #[test]
    fn token_new() {
        // arrange
        // act
        // assert
    }
}
