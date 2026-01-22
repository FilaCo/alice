use ac_db::db::AcDbTrait;
use ac_ir::{
    source::FileId,
    span::Span,
    token::{Base, LiteralKind, Token, TokenKind},
    value::Symbol,
};
use chumsky::{
    prelude::*, span::SimpleSpanned, text::{digits, ident, int}
};

use Base::*;
use LiteralKind::*;
use TokenKind::*;

// pub fn lexer<'db>(
    db: &'db dyn AcDbTrait,
    file: FileId<'db>,
) -> impl Parser<'db, &'db str, Vec<Token<'db>>, Rich<'db, char, Span<'db>>> {
    let line_comment = just("//").then(any().and_is(just('\n').not()).repeated());
    let ident = ident().map(|s| (Ident, Symbol::new(db, s)));

    let bin_int_lit = choice((just("0b"), just("0B")))
        .ignore_then(int(2))
        .map(|s| {
            (
                Literal {
                    kind: Int { base: Bin },
                },
                Symbol::new(db, s),
            )
        });

    let oct_int_lit = choice((just("0o"), just("0O")))
        .ignore_then(int(8))
        .map(|s| {
            (
                Literal {
                    kind: Int { base: Oct },
                },
                Symbol::new(db, s),
            )
        });

    let dec_int_lit = int(10);

    let hex_int_lit = choice((just("0x"), just("0X")))
        .ignore_then(int(16))
        .map(|s| {
            (
                Literal {
                    kind: Int { base: Hex },
                },
                Symbol::new(db, s),
            )
        });

    let int_lit = choice((bin_int_lit, oct_int_lit, hex_int_lit, dec_int_lit));

}

type ChumskyToken<'src> = (TokenKind, &'src str);

fn ac_ident<'src>() -> impl Parser<'src, &'src str, SimpleSpanned<ChumskyToken<'src>>, Rich<'src, char, SimpleSpan>> {
    ident().map(|s| (Ident, s))
}