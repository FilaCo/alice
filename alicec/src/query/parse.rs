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
        token::Token,
    },
};

const SRC: &str = r"
prop Foo {
    bar: int
}
";

#[salsa::tracked]
pub fn parse<'db>(db: &'db dyn AlicecDbTrait) -> Ast<'db> {
    todo!()
}

// fn parser<'db, I>() -> impl Parser<'db, I, Stmt<'db>>
// where
//     I: ValueInput<'db, Token = Token<'db>, Span = SimpleSpan>,
// {
//     todo!()
// }
