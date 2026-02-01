mod token;

pub use token::*;

use ac_lexer::Cursor;

pub struct Lexer<'src> {
    cursor: Cursor<'src>,
    pos: usize,
    token: Token<'src>,
}

impl<'src> Lexer<'src> {
    pub fn new(input: &'src str) -> Self {
        Self {
            cursor: Cursor::new(input),
            pos: 0,
            token: Token::dummy(),
        }
    }

    pub fn cook(&mut self) -> Token<'src> {
        todo!()
    }
}
