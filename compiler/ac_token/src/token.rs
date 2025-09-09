#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Token {
    kind: TokenKind,
    len: usize,
}

impl Token {
    #[inline]
    pub const fn kind(&self) -> TokenKind {
        self.kind
    }

    #[inline]
    #[allow(clippy::len_without_is_empty)]
    pub const fn len(&self) -> usize {
        self.len
    }

    #[inline]
    pub const fn line_comment(len: usize) -> Self {
        Self::new(TokenKind::LineComment, len)
    }

    #[inline]
    pub const fn block_comment(len: usize) -> Self {
        Self::new(TokenKind::BlockComment, len)
    }

    #[inline]
    pub const fn whitespace(len: usize) -> Self {
        Self::new(TokenKind::Whitespace, len)
    }

    #[inline]
    pub const fn ident(len: usize) -> Self {
        Self::new(TokenKind::Ident, len)
    }

    #[inline]
    pub const fn semi() -> Self {
        Self::single_char(TokenKind::Semi)
    }

    #[inline]
    pub const fn comma() -> Self {
        Self::single_char(TokenKind::Comma)
    }

    #[inline]
    pub const fn dot() -> Self {
        Self::single_char(TokenKind::Dot)
    }

    #[inline]
    pub const fn open_brace() -> Self {
        Self::single_char(TokenKind::OpenBrace)
    }

    #[inline]
    pub const fn close_brace() -> Self {
        Self::single_char(TokenKind::CloseBrace)
    }

    #[inline]
    pub const fn open_bracket() -> Self {
        Self::single_char(TokenKind::OpenBracket)
    }

    #[inline]
    pub const fn close_bracket() -> Self {
        Self::single_char(TokenKind::CloseBracket)
    }

    #[inline]
    pub const fn open_paren() -> Self {
        Self::single_char(TokenKind::OpenParen)
    }

    #[inline]
    pub const fn close_paren() -> Self {
        Self::single_char(TokenKind::CloseParen)
    }

    #[inline]
    pub const fn at() -> Self {
        Self::single_char(TokenKind::At)
    }

    #[inline]
    pub const fn hash() -> Self {
        Self::single_char(TokenKind::Hash)
    }

    #[inline]
    pub const fn tilde() -> Self {
        Self::single_char(TokenKind::Tilde)
    }

    #[inline]
    pub const fn quest() -> Self {
        Self::single_char(TokenKind::Quest)
    }

    #[inline]
    pub const fn colon() -> Self {
        Self::single_char(TokenKind::Colon)
    }

    #[inline]
    pub const fn dollar() -> Self {
        Self::single_char(TokenKind::Dollar)
    }

    #[inline]
    pub const fn eq() -> Self {
        Self::single_char(TokenKind::Eq)
    }

    #[inline]
    pub const fn ex() -> Self {
        Self::single_char(TokenKind::Ex)
    }

    #[inline]
    pub const fn lt() -> Self {
        Self::single_char(TokenKind::Lt)
    }

    #[inline]
    pub const fn gt() -> Self {
        Self::single_char(TokenKind::Gt)
    }

    #[inline]
    pub const fn minus() -> Self {
        Self::single_char(TokenKind::Minus)
    }

    #[inline]
    pub const fn amp() -> Self {
        Self::single_char(TokenKind::Amp)
    }

    #[inline]
    pub const fn pipe() -> Self {
        Self::single_char(TokenKind::Pipe)
    }

    #[inline]
    pub const fn plus() -> Self {
        Self::single_char(TokenKind::Plus)
    }

    #[inline]
    pub const fn star() -> Self {
        Self::single_char(TokenKind::Star)
    }

    #[inline]
    pub const fn slash() -> Self {
        Self::single_char(TokenKind::Slash)
    }

    #[inline]
    pub const fn caret() -> Self {
        Self::single_char(TokenKind::Caret)
    }

    #[inline]
    pub const fn percent() -> Self {
        Self::single_char(TokenKind::Percent)
    }

    #[inline]
    pub const fn unknown(len: usize) -> Self {
        Self::new(TokenKind::Unknown, len)
    }

    #[inline]
    const fn single_char(kind: TokenKind) -> Self {
        const SINGLE_CHAR_TOKEN_LEN: usize = 1;
        Self::new(kind, SINGLE_CHAR_TOKEN_LEN)
    }

    #[inline]
    const fn new(kind: TokenKind, len: usize) -> Self {
        Self { kind, len }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TokenKind {
    /// A line comment, e.g. `// comment`.
    LineComment,

    /// A block comment, e.g. `/* block comment */`.
    BlockComment,

    /// Any whitespace character sequence.
    Whitespace,

    /// An identifier or keyword, e.g. `ident` or `component`.
    Ident,

    /// `;`
    Semi,
    /// `,`
    Comma,
    /// `.`
    Dot,
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
    /// `@`
    At,
    /// `#`
    Hash,
    /// `~`
    Tilde,
    /// `?`
    Quest,
    /// `:`
    Colon,
    /// `$`
    Dollar,
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
    /// `*`
    Star,
    /// `/`
    Slash,
    /// `^`
    Caret,
    /// `%`
    Percent,

    /// Unknown token, not expected by the lexer, e.g. "№".
    Unknown,

    /// End of input.
    Eof,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_creates_line_comment() {
        // arrange
        let expected_len = 42;
        let expected_kind = TokenKind::LineComment;
        let expected = Token {
            kind: expected_kind,
            len: expected_len,
        };

        // act
        let actual = Token::line_comment(42);

        // assert
        assert_eq!(expected_kind, actual.kind());
        assert_eq!(expected_len, actual.len());
        assert_eq!(expected, actual);
    }

    #[test]
    fn it_creates_block_comment() {
        // arrange
        let expected_len = 0;
        let expected_kind = TokenKind::BlockComment;
        let expected = Token {
            kind: expected_kind,
            len: expected_len,
        };

        // act
        let actual = Token::block_comment(0);

        // assert
        assert_eq!(expected_kind, actual.kind());
        assert_eq!(expected_len, actual.len());
        assert_eq!(expected, actual);
    }

    #[test]
    fn it_creates_whitespace() {
        // arrange
        let expected_len = usize::MAX;
        let expected_kind = TokenKind::Whitespace;
        let expected = Token {
            kind: expected_kind,
            len: expected_len,
        };

        // act
        let actual = Token::whitespace(usize::MAX);

        // assert
        assert_eq!(expected_kind, actual.kind());
        assert_eq!(expected_len, actual.len());
        assert_eq!(expected, actual);
    }
}
