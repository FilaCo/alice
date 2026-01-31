mod token;

use std::{ops::Range, str::Chars};

use ac_lexer::Cursor;
pub use token::*;

pub type Span = Range<u32>;
pub type Spanned<T> = (T, Span);
const DUMMY_SPAN: Span = 0..0;

pub struct Lexer<'src> {
    cursor: Cursor<'src>,
    pos: usize,
    token: Spanned<Token<'src>>,
}

impl<'src> Lexer<'src> {
    pub fn new(input: &'src str) -> Self {
        Self {
            cursor: Cursor::new(input),
            pos: 0,
            token: Token::dummy(),
        }
    }

    pub fn cook(&mut self) -> Spanned<Token<'src>> {
        todo!()
    }
}

impl Token<'_> {
    pub const fn dummy() -> Spanned<Self> {
        (Token::Question, DUMMY_SPAN)
    }
}
