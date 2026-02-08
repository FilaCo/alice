use crate::lexer::Lexer;
use ac_db::db::AcDbTrait;
use ac_ir::syntax::{
    TokenType,
    ast::{AliceFile, TopLevelObject},
    token::Token,
};

use TokenType::*;

pub struct Parser<'db> {
    db: &'db dyn AcDbTrait,
    lexer: Lexer<'db>,
    cur: Token<'db>,
    prev: Token<'db>,
}

impl<'db> Parser<'db> {
    /// ```ebnf
    /// alice_file = { top_level_object } Eof
    /// ```
    pub fn alice_file(&mut self) -> AliceFile<'db> {
        let mut top_level_objs = vec![];
        while !self.seeing(Eof) {
            top_level_objs.push(self.top_level_object());
        }
        AliceFile::new(self.db, top_level_objs)
    }

    /// ```ebnf
    /// top_level_object = ( top_level_stmt | top_level_decl ) semis
    /// ```
    fn top_level_object(&mut self) -> TopLevelObject<'db> {
        todo!()
    }

    /// ```ebnf
    /// using_namespace = "using" "namespace" ident_path
    /// ```
    fn using_namespace(&mut self) {}

    fn eat(&mut self, tok_type: TokenType) -> bool {
        todo!()
    }

    fn seeing(&mut self, tok_type: TokenType) -> bool {
        todo!()
    }
}
