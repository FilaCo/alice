use crate::{cursor::*, token::*};

#[derive(Debug)]
pub struct Lexer<'src> {
    cursor: Cursor<'src>,
}

impl<'src> Lexer<'src> {
    pub fn new(input: &'src str) -> Self {
        let cursor = Cursor::new(input);
        Self { cursor }
    }

    pub fn cook(&mut self) -> Token {
        let Some(first_char) = self.cursor.bump() else {
            return Token::eof();
        };

        let kind = match first_char {
            SLASH_CHAR => match self.cursor.peek_first() {
                SLASH_CHAR => self.line_comment(),
                STAR_CHAR => self.block_comment(),
                _ => TokenKind::Slash,
            },

            c if is_whitespace(c) => self.whitespace(),

            c if is_id_start(c) => self.ident(),

            DOUBLE_QUOTE_CHAR => self.string_literal(),

            COMMA_CHAR => TokenKind::Comma,
            DOT_CHAR => TokenKind::Dot,
            EQ_CHAR => TokenKind::Eq,
            LT_CHAR => TokenKind::Lt,
            GT_CHAR => TokenKind::Gt,
            MINUS_CHAR => TokenKind::Minus,
            PLUS_CHAR => TokenKind::Plus,
            STAR_CHAR => TokenKind::Star,
            OPEN_BRACE_CHAR => TokenKind::OpenBrace,
            CLOSE_BRACE_CHAR => TokenKind::CloseBrace,
            OPEN_BRACKET_CHAR => TokenKind::OpenBracket,
            CLOSE_BRACKET_CHAR => TokenKind::CloseBracket,
            OPEN_PAREN_CHAR => TokenKind::OpenParen,
            CLOSE_PAREN_CHAR => TokenKind::CloseParen,

            _ => TokenKind::Unknown,
        };

        let res = Token::new(kind, self.token_len());
        self.cursor.reset_len_remaining();

        res
    }

    fn line_comment(&mut self) -> TokenKind {
        self.cursor.bump();

        self.cursor.bump_while(|c| !is_newline(c));
        TokenKind::LineComment
    }

    fn block_comment(&mut self) -> TokenKind {
        self.cursor.bump();

        let mut open_cnt = 1usize;
        while let Some(c) = self.cursor.bump() {
            match c {
                // `/*` branch
                SLASH_CHAR if self.cursor.peek_first() == STAR_CHAR => {
                    self.cursor.bump();
                    open_cnt += 1;
                }
                // `*/` branch
                STAR_CHAR if self.cursor.peek_first() == SLASH_CHAR => {
                    self.cursor.bump();
                    open_cnt -= 1;

                    if open_cnt == 0 {
                        break;
                    }
                }
                _ => (),
            }
        }

        TokenKind::BlockComment {
            is_terminated: if open_cnt == 0 {
                BlockCommentTerminated::Yes
            } else {
                BlockCommentTerminated::No
            },
        }
    }

    fn whitespace(&mut self) -> TokenKind {
        self.cursor.bump_while(is_whitespace);
        TokenKind::Whitespace
    }

    fn ident(&mut self) -> TokenKind {
        self.cursor.bump();
        self.cursor.bump_while(is_id_continue);
        TokenKind::Ident
    }

    fn string_literal(&mut self) -> TokenKind {
        self.cursor.bump();
        self.cursor
            .bump_while(|c| !is_newline(c) && !is_double_quote(c));

        let is_terminated = if is_double_quote(self.cursor.peek_first()) {
            self.cursor.bump();
            StringLiteralTerminated::Yes
        } else {
            StringLiteralTerminated::No
        };

        TokenKind::Literal {
            kind: LiteralKind::String { is_terminated },
        }
    }

    fn token_len(&self) -> usize {
        self.cursor.bumped_len()
    }
}
