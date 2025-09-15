#[derive(Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub len: usize,
}

#[derive(Debug)]
pub enum BlockCommentTerminated {
    No,
    Yes,
}

#[derive(Debug)]
pub enum StringLiteralTerminated {
    No,
    Yes,
}

#[derive(Debug)]
pub enum TokenKind {
    /// A line comment, e.g. `// comment`.
    LineComment,
    /// A block comment, e.g. `/* block comment */`.
    ///
    /// Block comments can be recursive, so a sequence like `/* /* */`
    /// will not be considered terminated and will result in a parsing error.
    BlockComment {
        is_terminated: BlockCommentTerminated,
    },

    /// Any whitespace character sequence.
    Whitespace,

    /// An identifier or keyword, e.g. `ident` or `component`.
    Ident,

    /// Literals.
    Literal { kind: LiteralKind },

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

#[derive(Debug)]
pub enum LiteralKind {
    String {
        is_terminated: StringLiteralTerminated,
    },
}
