use LiteralKind::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Token<'src> {
    /// Any whitespace character sequence.
    Whitespace,

    /// A line comment, e.g. `// comment`.
    LineComment,

    /// An identifier or keyword, e.g. `ident` or `prop`.
    Ident(&'src str),

    Literal(LiteralKind<'src>),

    /// `;`
    Semi,
    /// `,`
    Comma,
    /// `.`
    Dot,
    /// `{`
    LBrace,
    /// `}`
    RBrace,
    /// `[`
    LBracket,
    /// `]`
    RBracket,
    /// `(`
    LParen,
    /// `)`
    RParen,
    /// `:`
    Colon,
    /// `=`
    Eq,
    /// `!`
    Ex,
    /// `<`
    Lt,
    /// `>`
    Gt,
    /// `-`
    Minus,
    /// `+`
    Plus,
    /// `*`
    Star,
    /// `/`
    Slash,
    /// `%`
    Percent,

    /// Unknown token, not expected by the lexer, e.g. "№"
    Unknown,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum LiteralKind<'src> {
    Int { value: &'src str },
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use expect_test::{Expect, expect};

//     fn check_lexing(src: &str, expect: Expect) {
//         let tokens = Token::lexer(src)
//             .map(|item| match item {
//                 Ok(tok) => format!("{:?}\n", tok),
//                 Err(_) => format!("{:?}\n", Token::Unknown),
//             })
//             .collect::<String>();
//         expect.assert_eq(&tokens);
//     }

//     #[test]
//     fn smoke() {
//         let src = r"
// prop Point {
//     x: int
//     y: int
// }

// prop AddPoint {
//     other: id
// }

// sys PointAddPoint =
//     query Point as point, AddPoint as op
//     derive otherPoint = get<Point>(op.other)
//     derive {
//         point.x += otherPoint.x;
//         point.y += otherPoint.y;
//     }
//     delete<AddPoint>(self)
// ";
//         let expect = expect![[r#"
//             Ident("prop")
//             Ident("Point")
//             LBrace
//             Ident("x")
//             Colon
//             Ident("int")
//             Ident("y")
//             Colon
//             Ident("int")
//             RBrace
//             Ident("prop")
//             Ident("AddPoint")
//             LBrace
//             Ident("other")
//             Colon
//             Ident("id")
//             RBrace
//             Ident("sys")
//             Ident("PointAddPoint")
//             Eq
//             Ident("query")
//             Ident("Point")
//             Ident("as")
//             Ident("point")
//             Comma
//             Ident("AddPoint")
//             Ident("as")
//             Ident("op")
//             Ident("derive")
//             Ident("otherPoint")
//             Eq
//             Ident("get")
//             Lt
//             Ident("Point")
//             Gt
//             LParen
//             Ident("op")
//             Dot
//             Ident("other")
//             RParen
//             Ident("derive")
//             LBrace
//             Ident("point")
//             Dot
//             Ident("x")
//             Plus
//             Eq
//             Ident("otherPoint")
//             Dot
//             Ident("x")
//             Semi
//             Ident("point")
//             Dot
//             Ident("y")
//             Plus
//             Eq
//             Ident("otherPoint")
//             Dot
//             Ident("y")
//             Semi
//             RBrace
//             Ident("delete")
//             Lt
//             Ident("AddPoint")
//             Gt
//             LParen
//             Ident("self")
//             RParen
//             "#]];
//         check_lexing(src, expect);
//     }
// }
