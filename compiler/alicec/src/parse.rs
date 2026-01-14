use logos::Logos;

#[derive(Logos, Clone, Copy, Debug, PartialEq, Eq)]
enum Token<'source> {
    Whitespace,

    BlockComment(),
    LineComment,

    Ident(&'source str),

    LParen,
    RParen,

    Error,
}
