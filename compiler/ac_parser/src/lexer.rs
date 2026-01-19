use TokenKind::*;
use ac_ast::token::{Token, TokenKind};
use ac_db::{db::AcDbTrait, span::Span};
use ac_lexer::cursor::Cursor;

pub struct Lexer<'db> {
    db: &'db dyn AcDbTrait,
    src: &'db str,
    cursor: Cursor<'db>,
    pos: usize,
}

impl<'db> Lexer<'db> {
    pub fn new(db: &'db dyn AcDbTrait, src: &'db str) -> Self {
        let cursor = Cursor::new(src);
        let pos = 0;

        Self {
            db,
            src,
            cursor,
            pos,
        }
    }

    pub fn advance_token(&mut self) -> Token<'db> {
        let mut swallow_next_invalid = 0;
        loop {
            let raw_lexeme = self.cursor.advance_token();
            let start = self.pos;
            self.pos += raw_lexeme.len;

            let kind = match raw_lexeme.kind {
                ac_lexer::token::TokenKind::Eof => Eof,

                ac_lexer::token::TokenKind::Unknown => {
                    if swallow_next_invalid > 0 {
                        swallow_next_invalid -= 1;
                        continue;
                    }

                    let mut it = self.str_from_to_end(start).chars();
                    let c = it.next().expect("unable to get next char");
                    swallow_next_invalid = it.take_while(|c1| *c1 == c).count();

                    Diagnostic::unknown_token_start().accumulate(self.db);

                    continue;
                }
            };

            let span = Span::new(self.db, start.into(), self.pos.into());
            return Token { kind, span };
        }
    }
}
