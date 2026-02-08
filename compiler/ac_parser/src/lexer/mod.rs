use ac_db::db::AcDbTrait;
use ac_ir::{
    source::Span,
    syntax::token::{Token, TokenKind},
};

use crate::lexer::cursor::Cursor;
use TokenKind::*;

mod cursor;

pub struct Lexer<'db> {
    db: &'db dyn AcDbTrait,
    cursor: Cursor<'db>,
    token: Token<'db>,
    pos: usize,
}

impl<'db> Lexer<'db> {
    pub fn new(db: &'db dyn AcDbTrait, input: &'db str) -> Self {
        Self {
            db,
            cursor: Cursor::new(input),
            token: Token::dummy(db),
            pos: 0,
        }
    }

    pub fn cook(&mut self) -> Token<'db> {
        todo!()
    }
}
