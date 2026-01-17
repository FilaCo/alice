use chumsky::{
    Parser,
    input::{Stream, ValueInput},
    span::SimpleSpan,
};
use logos::Logos;

use crate::{
    db::AlicecDbTrait,
    ir::{
        ast::{Ast, Stmt},
        source::SourceFile,
        token::Token,
    },
};

#[salsa::tracked]
pub fn parse<'db>(db: &'db dyn AlicecDbTrait, src: SourceFile<'db>) -> Ast<'db> {
    todo!()
}

// fn parser<'db, I>() -> impl Parser<'db, I, Stmt<'db>>
// where
//     I: ValueInput<'db, Token = Token<'db>, Span = SimpleSpan>,
// {
//     todo!()
// }
