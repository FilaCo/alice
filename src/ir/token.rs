use logos::Logos;

#[derive(Logos, Debug, PartialEq, Eq, Clone, Copy)]
pub enum Token {
    /// Any whitespace character sequence.
    #[regex(r"[ \t\n\r\f]+", logos::skip)]
    Whitespace,

    Ident,

    Lit,

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

    Error,
}
