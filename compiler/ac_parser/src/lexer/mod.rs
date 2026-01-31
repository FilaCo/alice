mod token;

use std::{ops::Range, str::Chars};

pub use token::*;

pub type Span = Range<u32>;
pub type Spanned<T> = (T, Span);
const DUMMY_SPAN: Span = 0..0;

pub struct Lexer<'src> {
    src: &'src str,
    cursor: Chars<'src>,
    pos: usize,
    cur: Spanned<Token<'src>>,
}

impl<'src> Lexer<'src> {
    pub fn new(src: &'src str) -> Self {
        Self {
            src,
            cursor: src.chars(),
            pos: 0,
            cur: Token::dummy(),
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
