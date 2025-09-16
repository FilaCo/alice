use alicec_db::prelude::AliceDbTrait;
use alicec_lexer::prelude::{Lexer as RawTokenLexer, Terminated, TokenKind as RawTokenKind};
use alicec_token::prelude::Token;

pub struct Lexer<'db> {
    db: &'db dyn AliceDbTrait,
    raw_token_lexer: RawTokenLexer<'db>,
    pos: usize,
}

impl<'db> Lexer<'db> {
    pub fn new(db: &'db dyn AliceDbTrait, input: &'db str) -> Self {
        let cursor = RawTokenLexer::new(input);
        Self {
            db,
            raw_token_lexer: cursor,
            pos: 0,
        }
    }

    pub fn cook(&mut self) -> Token<'db> {
        todo!()
    }
}

#[derive(Debug)]
enum PreceededByWhitespace {
    No,
    Yes,
}
