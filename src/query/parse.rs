use chumsky::input::Stream;
use logos::Logos;

use crate::{
    db::{AliceDb, AliceDbTrait},
    ir::{Ast, SourceProgram},
};

#[salsa::tracked]
pub fn parse_query(db: &dyn AliceDbTrait, src: SourceProgram) -> Ast<'_> {
    // let src_text = src.text(db);
    // let token_iter = Token::lexer(src_text).spanned().map(|(t, s)| match t {
    // Ok(tok) => (tok, s.into()),
    // Err(_) => (Token::Error, s.into()),
    // });

    // let token_stream =
    // Stream::from_iter(token_iter).map((0..src_text.len()).into(), |(t, s): (_, _)| (t, s));
    todo!()
}

#[derive(Logos, Debug, PartialEq, Eq, Clone, Copy)]
enum Token {
    /// Any whitespace character sequence.
    #[regex(r"[ \t\n\r\f]+", logos::skip)]
    Whitespace,

    Error,

    /// `;`
    #[token(";")]
    Semi,
    /// `,`
    #[token(",")]
    Comma,
    /// `.`
    #[token(".")]
    Dot,
    /// `{`
    #[token("{")]
    OpenBrace,
    /// `}`
    #[token("}")]
    CloseBrace,
    /// `[`
    #[token("[")]
    OpenBracket,
    /// `]`
    #[token("]")]
    CloseBracket,
    /// `(`
    #[token("(")]
    OpenParen,
    /// `)`
    #[token(")")]
    CloseParen,
    /// `@`
    #[token("@")]
    At,
    /// `#`
    #[token("#")]
    Hash,
    /// `~`
    #[token("~")]
    Tilde,
    /// `?`
    #[token("?")]
    Question,
    /// `:`
    #[token(":")]
    Colon,
    /// `$`
    #[token("$")]
    Dollar,
    /// `=`
    #[token("=")]
    Eq,
    /// `!`
    #[token("!")]
    Exclamation,
    /// `<`
    #[token("<")]
    Lt,
    /// `>`
    #[token(">")]
    Gt,
    /// `-`
    #[token("-")]
    Minus,
    /// `&`
    #[token("&")]
    Amp,
    /// `|`
    #[token("|")]
    Pipe,
    /// `+`
    #[token("+")]
    Plus,
    /// `*`
    #[token("*")]
    Star,
    /// `/`
    #[token("/")]
    Slash,
    /// `^`
    #[token("^")]
    Caret,
    /// `%`
    #[token("%")]
    Percent,
}
