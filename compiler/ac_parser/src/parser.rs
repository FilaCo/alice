use crate::lexer::{Lexer, Spanned, Token};
use ac_db::db::AcDbTrait;

pub struct Parser<'db> {
    db: &'db dyn AcDbTrait,
    lexer: Lexer<'db>,
    cur: Spanned<Token<'db>>,
    prev: Spanned<Token<'db>>,
}

impl<'db> Parser<'db> {
    pub(crate) fn bump(&mut self) {
        self.prev = std::mem::replace(&mut self.cur, self.lexer.cook());
    }
}
