use alicec_db::prelude::AliceDbTrait;
use alicec_lexer::prelude::{
    BlockCommentTerminated, Lexer as RawTokenLexer, TokenKind as RawTokenKind,
};
use alicec_token::prelude::{Token, TokenKind};

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
        let mut preceded_by_whitespace = PreceededByWhitespace::No;

        loop {
            let raw_token = self.raw_token_lexer.cook();
            let start = self.pos;
            self.pos = raw_token.len;

            let kind = match raw_token.kind {
                RawTokenKind::LineComment => {
                    preceded_by_whitespace = PreceededByWhitespace::Yes;
                    continue;
                }
                RawTokenKind::BlockComment { is_terminated } => {
                    if matches!(is_terminated, BlockCommentTerminated::No) {}

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

            return Token::new(kind);
        }
    }
}

#[derive(Debug)]
enum PreceededByWhitespace {
    No,
    Yes,
}
