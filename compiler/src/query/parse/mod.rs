mod lexer;

use std::str::Chars;

use crate::{
    db::{AliceDbTrait, Symbol},
    ir::{Ast, SourceCode},
    span::Span,
};

#[salsa::tracked]
pub fn parse(db: &dyn AliceDbTrait, src: SourceCode) -> Ast<'_> {
    Parser::new(db, Lexer::new(db, src)).parse()
}

struct Parser<'db> {
    db: &'db dyn AliceDbTrait,
    lexer: Lexer<'db>,
}

struct Lexer<'db> {
    db: &'db dyn AliceDbTrait,
    chars: Chars<'db>,
    len_remaining: usize,
    pos: usize,
}

impl<'db> Parser<'db> {
    pub const fn new(db: &'db dyn AliceDbTrait, lexer: Lexer<'db>) -> Self {
        Self { db, lexer }
    }

    pub fn parse(&mut self) -> Ast<'db> {
        todo!()
    }
}

impl<'db> Lexer<'db> {
    pub fn new(db: &'db dyn AliceDbTrait, src: SourceCode) -> Self {
        let contents = src.contents(db);
        let chars = contents.chars();
        let len_remaining = contents.len();
        Self {
            db,
            chars,
            len_remaining,
            pos: 0,
        }
    }

    pub fn cook(&mut self) -> Token<'db> {
        todo!()
    }
}

struct Token<'db> {
    kind: TokenKind<'db>,
    span: Span<'db>,
}

#[derive(Debug)]
enum TokenKind<'db> {
    /// `{`.
    OpenBrace,
    /// `}`.
    CloseBrace,
    /// `(`.
    OpenParen,
    /// `)`.
    CloseParen,

    /// An identifier or keyword, e.g. `foo` or `component`.
    Ident(Symbol<'db>),
    Literal,

    /// End of file.
    Eof,
}

impl<'db> Token<'db> {
    pub const fn eof(span: Span<'db>) -> Self {
        Self {
            kind: TokenKind::Eof,
            span,
        }
    }
}
