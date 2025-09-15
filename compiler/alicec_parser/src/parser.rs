use crate::lexer::Lexer;
use alicec_db::prelude::AliceDbTrait;

pub struct Parser<'db> {
    lexer: Lexer<'db>,
}

impl<'db> Parser<'db> {
    pub fn new(lexer: Lexer<'db>) -> Self {
        Self { lexer }
    }
}
