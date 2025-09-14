use crate::query::parse::lexer::raw_token::LiteralKind;

use super::raw_token::{BlockCommentTerminated, RawToken, RawTokenKind, StringLiteralTerminated};
use std::str::Chars;

#[derive(Debug)]
pub struct Cursor<'src> {
    len_remaining: usize,
    chars: Chars<'src>,
}

impl<'src> Cursor<'src> {
    pub fn new(input: &'src str) -> Self {
        Self {
            chars: input.chars(),
            len_remaining: input.len(),
        }
    }

    pub fn bump_raw_token(&mut self) -> RawToken {
        let Some(first_char) = self.bump() else {
            return RawToken::eof();
        };

        let kind = match first_char {
            SLASH_CHAR => match self.peek_first() {
                SLASH_CHAR => self.line_comment(),
                STAR_CHAR => self.block_comment(),
                _ => RawTokenKind::Slash,
            },

            c if is_whitespace(c) => self.whitespace(),

            c if is_id_start(c) => self.ident(),

            DOUBLE_QUOTE_CHAR => self.string_literal(),

            COMMA_CHAR => RawTokenKind::Comma,
            DOT_CHAR => RawTokenKind::Dot,
            EQ_CHAR => RawTokenKind::Eq,
            LT_CHAR => RawTokenKind::Lt,
            GT_CHAR => RawTokenKind::Gt,
            MINUS_CHAR => RawTokenKind::Minus,
            PLUS_CHAR => RawTokenKind::Plus,
            STAR_CHAR => RawTokenKind::Star,
            OPEN_BRACE_CHAR => RawTokenKind::OpenBrace,
            CLOSE_BRACE_CHAR => RawTokenKind::CloseBrace,
            OPEN_BRACKET_CHAR => RawTokenKind::OpenBracket,
            CLOSE_BRACKET_CHAR => RawTokenKind::CloseBracket,
            OPEN_PAREN_CHAR => RawTokenKind::OpenParen,
            CLOSE_PAREN_CHAR => RawTokenKind::CloseParen,

            _ => RawTokenKind::Unknown,
        };

        let res = RawToken::new(kind, self.token_len());
        self.reset_len_remaining();

        res
    }

    fn line_comment(&mut self) -> RawTokenKind {
        self.bump();

        self.bump_while(|c| !is_newline(c));
        RawTokenKind::LineComment
    }

    fn block_comment(&mut self) -> RawTokenKind {
        self.bump();

        let mut open_cnt = 1u64;
        while let Some(c) = self.bump() {
            match c {
                // `/*` branch
                SLASH_CHAR if self.peek_first() == STAR_CHAR => {
                    self.bump();
                    open_cnt += 1;
                }
                // `*/` branch
                STAR_CHAR if self.peek_first() == SLASH_CHAR => {
                    self.bump();
                    open_cnt -= 1;

                    if open_cnt == 0 {
                        break;
                    }
                }
                _ => (),
            }
        }

        RawTokenKind::BlockComment {
            is_terminated: if open_cnt == 0 {
                BlockCommentTerminated::Yes
            } else {
                BlockCommentTerminated::No
            },
        }
    }

    fn whitespace(&mut self) -> RawTokenKind {
        self.bump_while(is_whitespace);
        RawTokenKind::Whitespace
    }

    fn ident(&mut self) -> RawTokenKind {
        self.bump();
        self.bump_while(is_id_continue);
        RawTokenKind::Ident
    }

    fn string_literal(&mut self) -> RawTokenKind {
        self.bump();
        self.bump_while(|c| !is_newline(c) && !is_double_quote(c));

        let is_terminated = if is_double_quote(self.peek_first()) {
            self.bump();
            StringLiteralTerminated::Yes
        } else {
            StringLiteralTerminated::No
        };

        RawTokenKind::Literal {
            kind: LiteralKind::String { is_terminated },
        }
    }

    fn is_eof(&self) -> bool {
        self.chars.as_str().is_empty()
    }

    fn bump(&mut self) -> Option<char> {
        self.chars.next()
    }

    fn bump_while(&mut self, predicate: impl Fn(char) -> bool) {
        while predicate(self.peek_first()) && !self.is_eof() {
            self.bump();
        }
    }

    fn peek_first(&self) -> char {
        self.chars.clone().next().unwrap_or(EOF_CHAR)
    }

    fn peek_second(&self) -> char {
        let mut iter = self.chars.clone();
        iter.next();
        iter.next().unwrap_or(EOF_CHAR)
    }

    fn token_len(&self) -> usize {
        self.len_remaining - self.chars.as_str().len()
    }

    fn reset_len_remaining(&mut self) {
        self.len_remaining = self.chars.as_str().len();
    }
}

const COMMA_CHAR: char = ',';
const DOT_CHAR: char = '.';
const EQ_CHAR: char = '=';
const LT_CHAR: char = '<';
const GT_CHAR: char = '>';
const MINUS_CHAR: char = '-';
const PLUS_CHAR: char = '+';
const SLASH_CHAR: char = '/';
const STAR_CHAR: char = '*';
const OPEN_BRACE_CHAR: char = '{';
const CLOSE_BRACE_CHAR: char = '}';
const OPEN_BRACKET_CHAR: char = '[';
const CLOSE_BRACKET_CHAR: char = ']';
const OPEN_PAREN_CHAR: char = '(';
const CLOSE_PAREN_CHAR: char = ')';

const DOUBLE_QUOTE_CHAR: char = '"';

const NEW_LINE_CHAR: char = '\n';
const TAB_CHAR: char = '\t';
const CARRIAGE_RETURN_CHAR: char = '\r';
const SPACE_CHAR: char = ' ';

const EOF_CHAR: char = '\0';

fn is_double_quote(c: char) -> bool {
    c == DOUBLE_QUOTE_CHAR
}

fn is_id_start(c: char) -> bool {
    c == '_' || unicode_xid::UnicodeXID::is_xid_start(c)
}

fn is_id_continue(c: char) -> bool {
    unicode_xid::UnicodeXID::is_xid_continue(c)
}

fn is_whitespace(c: char) -> bool {
    is_newline(c) || is_tab(c) || is_carriage_return(c) || is_space(c)
}

fn is_newline(c: char) -> bool {
    c == NEW_LINE_CHAR
}

fn is_tab(c: char) -> bool {
    c == TAB_CHAR
}

fn is_carriage_return(c: char) -> bool {
    c == CARRIAGE_RETURN_CHAR
}

fn is_space(c: char) -> bool {
    c == SPACE_CHAR
}
