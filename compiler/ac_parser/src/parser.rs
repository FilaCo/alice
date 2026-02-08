use crate::lexer::Lexer;
use ac_db::db::AcDbTrait;
use ac_ir::syntax::token::Token;

pub struct Parser<'db> {
    db: &'db dyn AcDbTrait,
    lexer: Lexer<'db>,
    cur: Token<'db>,
    prev: Token<'db>,
}

impl<'db> Parser<'db> {
    pub(crate) fn bump(&mut self) {
        self.prev = std::mem::replace(&mut self.cur, self.lexer.cook());
    }
}
