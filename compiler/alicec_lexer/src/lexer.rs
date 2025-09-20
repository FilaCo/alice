use crate::cursor::Cursor;
use crate::token::{LitKind as RawLitKind, TokenKind as RawTokenKind};
use alicec_db::prelude::{AlicecDbTrait, Diagnostic, Span, Spanned, Symbol};
use alicec_ir::prelude::{LitKind, Token};

use LitKind::*;
use Token::*;

pub struct Lexer<'db> {
    db: &'db dyn AlicecDbTrait,
    src: &'db str,
    cursor: Cursor<'db>,
    pos: usize,
}

impl<'db> Lexer<'db> {
    pub fn new(db: &'db dyn AlicecDbTrait, src: &'db str) -> Self {
        let cursor = Cursor::new(src);
        let pos = 0;

        Self {
            db,
            src,
            cursor,
            pos,
        }
    }

    pub fn cook(&mut self) -> Spanned<'db, Token<'db>> {
        let mut swallow_next_invalid = 0;
        loop {
            let raw_lexeme = self.cursor.bump_token();
            let start = self.pos;
            self.pos += raw_lexeme.len;

            let lexeme = match raw_lexeme.kind {
                RawTokenKind::Whitespace => continue,

                RawTokenKind::Lit(lit_kind) => self.cook_literal(lit_kind, start, self.pos),

                RawTokenKind::Minus => Minus,
                RawTokenKind::Plus => Plus,
                RawTokenKind::Slash => Slash,
                RawTokenKind::Star => Star,
                RawTokenKind::LParen => LParen,
                RawTokenKind::RParen => RParen,

                RawTokenKind::Eof => Eof,

                _ => {
                    if swallow_next_invalid > 0 {
                        swallow_next_invalid -= 1;
                        continue;
                    }

                    let mut it = self.str_from_to_end(start).chars();
                    let c = it.next().expect("unable to get next char");
                    swallow_next_invalid = it.take_while(|c1| *c1 == c).count();

                    Diagnostic::unknown_token_start(self.db);

                    continue;
                }
            };

            let span = Span::new(self.db, start, self.pos);
            return Spanned::new(lexeme, span);
        }
    }

    fn cook_literal(&self, raw_kind: RawLitKind, start: usize, end: usize) -> Token<'db> {
        let kind = match raw_kind {
            RawLitKind::Int => Int,
            RawLitKind::Float => Float,
        };
        let symbol = Symbol::new(self.db, self.str_from_to(start, end));
        Lit { kind, symbol }
    }

    fn str_from_to_end(&self, start: usize) -> &str {
        &self.src[start..]
    }

    fn str_from_to(&self, start: usize, end: usize) -> &str {
        &self.src[start..end]
    }
}
