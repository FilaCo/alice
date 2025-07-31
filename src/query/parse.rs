use chumsky::input::Stream;
use logos::Logos;
use salsa::Accumulator;

use crate::prelude::*;

#[salsa::tracked]
pub fn parse(db: &dyn AliceDbTrait, src: SourceProgram) -> Ast<'_> {
    let src_text = src.text(db);

    // let token_iter = Token::lexer(src_text).spanned().map(|(t, s)| match t {
    //     Ok(t) => (t, s.into()),
    //     Err(_) => (Token::Error, s.into()),
    // });

    // let token_stream =
    // Stream::from_iter(token_iter).map((0..src_text.len()).into(), |(t, s): (_, _)| (t, s));

    Ast::new(db)
}
