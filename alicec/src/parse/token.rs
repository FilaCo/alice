use logos::Logos;

use LitKind::*;

#[derive(Logos, Clone, Copy, Debug, PartialEq, Eq)]
pub enum Token<'src> {
    /// Any whitespace character sequence.
    #[regex(r"[ \t\n\r]+", logos::skip)]
    Whitespace,

    /// Any whitespace character sequence.
    #[regex(r"//.*", logos::skip, allow_greedy = true)]
    LineComment,

    /// An identifier or keyword, e.g. `ident` or `prop`.
    #[regex(r"[A-Za-z_][A-Za-z0-9_]*")]
    Ident(&'src str),

    #[regex(r"[0-9]+(?:_[0-9]+)*", |lex| Int { value: lex.slice() })]
    Lit(LitKind<'src>),

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
    LBrace,
    /// `}`
    #[token("}")]
    RBrace,
    /// `[`
    #[token("[")]
    LBracket,
    /// `]`
    #[token("]")]
    RBracket,
    /// `(``
    #[token("(")]
    LParen,
    /// `)`
    #[token(")")]
    RParen,
    /// `:`
    #[token(":")]
    Colon,
    /// `=`
    #[token("=")]
    Eq,
    /// `!`
    #[token("!")]
    Ex,
    /// `<`
    #[token("<")]
    Lt,
    /// `>`
    #[token(">")]
    Gt,
    /// `-`
    #[token("-")]
    Minus,
    /// `+`
    #[token("+")]
    Plus,
    /// `*`
    #[token("*")]
    Star,
    /// `/`
    #[token("/")]
    Slash,
    /// `%`
    #[token("%")]
    Percent,

    /// Unknown token, not expected by the lexer, e.g. "№"
    Error,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum LitKind<'src> {
    Int { value: &'src str },
}

#[cfg(test)]
mod tests {
    use super::*;
    use expect_test::{Expect, expect};

    fn check_lexing(src: &str, expect: Expect) {
        let tokens = Token::lexer(src)
            .map(|item| match item {
                Ok(tok) => format!("{:?}\n", tok),
                Err(_) => format!("{:?}\n", Token::Error),
            })
            .collect::<String>();
        expect.assert_eq(&tokens);
    }

    #[test]
    fn smoke() {
        let src = r"
prop Point {
    x: int
    y: int
}

prop AddPoint {
    other: Entity
}

sys PointAddPoint =
    query Point as point, AddPoint as op
    derive otherPoint = fetch<Point>(op.other)
    derive {
        point.x += otherPoint.x;
        point.y += otherPoint.y;
    }
    delete op
";
        let expect = expect![[r#"
            Ident("prop")
            Ident("Point")
            LBrace
            Ident("x")
            Colon
            Ident("int")
            Ident("y")
            Colon
            Ident("int")
            RBrace
            Ident("prop")
            Ident("AddPoint")
            LBrace
            Ident("other")
            Colon
            Ident("Entity")
            RBrace
            Ident("sys")
            Ident("PointAddPoint")
            Eq
            Ident("query")
            Ident("Point")
            Ident("as")
            Ident("point")
            Comma
            Ident("AddPoint")
            Ident("as")
            Ident("op")
            Ident("derive")
            Ident("otherPoint")
            Eq
            Ident("fetch")
            Lt
            Ident("Point")
            Gt
            LParen
            Ident("op")
            Dot
            Ident("other")
            RParen
            Ident("derive")
            LBrace
            Ident("point")
            Dot
            Ident("x")
            Plus
            Eq
            Ident("otherPoint")
            Dot
            Ident("x")
            Semi
            Ident("point")
            Dot
            Ident("y")
            Plus
            Eq
            Ident("otherPoint")
            Dot
            Ident("y")
            Semi
            RBrace
            Ident("delete")
            Ident("op")
            "#]];
        check_lexing(src, expect);
    }
}
