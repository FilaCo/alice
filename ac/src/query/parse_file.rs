use chumsky::{
    Parser,
    input::{Stream, ValueInput},
    span::SimpleSpan,
};
use logos::Logos;

use crate::{
    db::AcDbTrait,
    ir::{
        ast::{Ast, Stmt},
        source::SourceFile,
        token::Token,
    },
};

#[salsa::tracked]
pub fn parse_file<'db>(db: &'db dyn AcDbTrait, src: SourceFile) -> Ast<'db> {
    todo!()
}

// fn parser<'db, I>() -> impl Parser<'db, I, Stmt<'db>>
// where
//     I: ValueInput<'db, Token = Token<'db>, Span = SimpleSpan>,
// {
//     todo!()
// }
