mod lexer;

use crate::{
    db::{AliceDbTrait, Symbol},
    ir::{Ast, SourceCode},
};
use lexer::Lexer;

#[salsa::tracked]
pub fn parse(db: &dyn AliceDbTrait, src: SourceCode) -> Ast<'_> {
    Parser::new(db, Lexer::new(db, src)).parse()
}

struct Parser<'db> {
    db: &'db dyn AliceDbTrait,
    lexer: Lexer<'db>,
}

impl<'db> Parser<'db> {
    pub const fn new(db: &'db dyn AliceDbTrait, lexer: Lexer<'db>) -> Self {
        Self { db, lexer }
    }

    pub fn parse(&mut self) -> Ast<'db> {
        todo!()
    }
}
