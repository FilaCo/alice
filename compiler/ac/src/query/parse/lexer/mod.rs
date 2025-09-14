mod cursor;
mod raw_token;

use crate::{
    db::AliceDbTrait,
    diagnostic::{Diagnostic, Span},
    ir::{SourceCode, Token, TokenKind},
    query::parse::lexer::raw_token::BlockCommentTerminated,
};
use cursor::Cursor;
use raw_token::RawTokenKind;
use salsa::Accumulator;

pub struct Lexer<'db> {
    db: &'db dyn AliceDbTrait,
    cursor: Cursor<'db>,
    pos: usize,
}

#[derive(Debug)]
enum PreceededByWhitespace {
    No,
    Yes,
}

impl<'db> Lexer<'db> {
    pub fn new(db: &'db dyn AliceDbTrait, src: SourceCode) -> Self {
        let contents = src.contents(db);
        let cursor = Cursor::new(contents);
        Self { db, cursor, pos: 0 }
    }

    pub fn cook(&mut self) -> Token<'db> {
        let mut preceded_by_whitespace = PreceededByWhitespace::No;

        loop {
            let raw_token = self.cursor.bump_raw_token();
            let start = self.pos;
            self.pos = raw_token.len;

            let kind = match raw_token.kind {
                RawTokenKind::LineComment => {
                    preceded_by_whitespace = PreceededByWhitespace::Yes;
                    continue;
                }
                RawTokenKind::BlockComment { is_terminated } => {
                    if matches!(is_terminated, BlockCommentTerminated::No) {
                        Diagnostic::new(kind, msg, start, self.pos, vec![]).accumulate(self.db);
                    }

                    preceded_by_whitespace = PreceededByWhitespace::Yes;
                    continue;
                }
                RawTokenKind::Whitespace => {
                    preceded_by_whitespace = PreceededByWhitespace::Yes;
                    continue;
                }
                RawTokenKind::Ident => todo!(),
                RawTokenKind::Literal { kind } => todo!(),
                RawTokenKind::Comma => todo!(),
                RawTokenKind::Dot => todo!(),
                RawTokenKind::Eq => todo!(),
                RawTokenKind::Lt => todo!(),
                RawTokenKind::Gt => todo!(),
                RawTokenKind::Minus => todo!(),
                RawTokenKind::Plus => todo!(),
                RawTokenKind::Slash => todo!(),
                RawTokenKind::Star => todo!(),
                RawTokenKind::OpenBrace => TokenKind::OpenBrace,
                RawTokenKind::CloseBrace => TokenKind::CloseBrace,
                RawTokenKind::OpenBracket => TokenKind::OpenBracket,
                RawTokenKind::CloseBracket => TokenKind::CloseBracket,
                RawTokenKind::OpenParen => TokenKind::OpenParen,
                RawTokenKind::CloseParen => TokenKind::CloseParen,
                RawTokenKind::Unknown => todo!(),
                RawTokenKind::Eof => TokenKind::Eof,
            };

            let span = Span::new(self.db, start, self.pos);
            return Token::new(kind, span);
        }
    }
}
