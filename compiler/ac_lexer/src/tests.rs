use super::*;
use expect_test::{Expect, expect};

impl<'src> Cursor<'src> {
    pub fn tokenize(input: &'src str) -> impl Iterator<Item = Token> {
        let mut cursor = Self::new(input);

        std::iter::from_fn(move || {
            let lexeme = cursor.advance_token();

            if lexeme.kind != Eof {
                Some(lexeme)
            } else {
                None
            }
        })
    }
}

fn check_lexing(src: &str, expect: Expect) {
    let actual: String = Cursor::tokenize(src)
        .map(|token| format!("{:?}\n", token))
        .collect();
    expect.assert_eq(&actual)
}

#[test]
fn smoke() {
    let src = r"
prop Point {
    x: int
    y: int
}

prop AddPoint {
    other: id
}

sys PointAddPoint =
    query Point as point, AddPoint as op
    derive otherPoint = get<Point>(op.other)
    derive {
        point.x += otherPoint.x;
        point.y += otherPoint.y;
    }
    delete<AddPoint>(self)
";
    let expect = expect![[r#"
        Token { kind: Whitespace, len: 1 }
        Token { kind: Ident, len: 4 }
        Token { kind: Whitespace, len: 1 }
        Token { kind: Ident, len: 5 }
        Token { kind: Whitespace, len: 1 }
        Token { kind: LBrace, len: 1 }
        Token { kind: Whitespace, len: 5 }
        Token { kind: Ident, len: 1 }
        Token { kind: Colon, len: 1 }
        Token { kind: Whitespace, len: 1 }
        Token { kind: Ident, len: 3 }
        Token { kind: Whitespace, len: 5 }
        Token { kind: Ident, len: 1 }
        Token { kind: Colon, len: 1 }
        Token { kind: Whitespace, len: 1 }
        Token { kind: Ident, len: 3 }
        Token { kind: Whitespace, len: 1 }
        Token { kind: RBrace, len: 1 }
        Token { kind: Whitespace, len: 2 }
        Token { kind: Ident, len: 4 }
        Token { kind: Whitespace, len: 1 }
        Token { kind: Ident, len: 8 }
        Token { kind: Whitespace, len: 1 }
        Token { kind: LBrace, len: 1 }
        Token { kind: Whitespace, len: 5 }
        Token { kind: Ident, len: 5 }
        Token { kind: Colon, len: 1 }
        Token { kind: Whitespace, len: 1 }
        Token { kind: Ident, len: 2 }
        Token { kind: Whitespace, len: 1 }
        Token { kind: RBrace, len: 1 }
        Token { kind: Whitespace, len: 2 }
        Token { kind: Ident, len: 3 }
        Token { kind: Whitespace, len: 1 }
        Token { kind: Ident, len: 13 }
        Token { kind: Whitespace, len: 1 }
        Token { kind: Eq, len: 1 }
        Token { kind: Whitespace, len: 5 }
        Token { kind: Ident, len: 5 }
        Token { kind: Whitespace, len: 1 }
        Token { kind: Ident, len: 5 }
        Token { kind: Whitespace, len: 1 }
        Token { kind: Ident, len: 2 }
        Token { kind: Whitespace, len: 1 }
        Token { kind: Ident, len: 5 }
        Token { kind: Comma, len: 1 }
        Token { kind: Whitespace, len: 1 }
        Token { kind: Ident, len: 8 }
        Token { kind: Whitespace, len: 1 }
        Token { kind: Ident, len: 2 }
        Token { kind: Whitespace, len: 1 }
        Token { kind: Ident, len: 2 }
        Token { kind: Whitespace, len: 5 }
        Token { kind: Ident, len: 6 }
        Token { kind: Whitespace, len: 1 }
        Token { kind: Ident, len: 10 }
        Token { kind: Whitespace, len: 1 }
        Token { kind: Eq, len: 1 }
        Token { kind: Whitespace, len: 1 }
        Token { kind: Ident, len: 3 }
        Token { kind: Lt, len: 1 }
        Token { kind: Ident, len: 5 }
        Token { kind: Gt, len: 1 }
        Token { kind: LParen, len: 1 }
        Token { kind: Ident, len: 2 }
        Token { kind: Dot, len: 1 }
        Token { kind: Ident, len: 5 }
        Token { kind: RParen, len: 1 }
        Token { kind: Whitespace, len: 5 }
        Token { kind: Ident, len: 6 }
        Token { kind: Whitespace, len: 1 }
        Token { kind: LBrace, len: 1 }
        Token { kind: Whitespace, len: 9 }
        Token { kind: Ident, len: 5 }
        Token { kind: Dot, len: 1 }
        Token { kind: Ident, len: 1 }
        Token { kind: Whitespace, len: 1 }
        Token { kind: Plus, len: 1 }
        Token { kind: Eq, len: 1 }
        Token { kind: Whitespace, len: 1 }
        Token { kind: Ident, len: 10 }
        Token { kind: Dot, len: 1 }
        Token { kind: Ident, len: 1 }
        Token { kind: Semi, len: 1 }
        Token { kind: Whitespace, len: 9 }
        Token { kind: Ident, len: 5 }
        Token { kind: Dot, len: 1 }
        Token { kind: Ident, len: 1 }
        Token { kind: Whitespace, len: 1 }
        Token { kind: Plus, len: 1 }
        Token { kind: Eq, len: 1 }
        Token { kind: Whitespace, len: 1 }
        Token { kind: Ident, len: 10 }
        Token { kind: Dot, len: 1 }
        Token { kind: Ident, len: 1 }
        Token { kind: Semi, len: 1 }
        Token { kind: Whitespace, len: 5 }
        Token { kind: RBrace, len: 1 }
        Token { kind: Whitespace, len: 5 }
        Token { kind: Ident, len: 6 }
        Token { kind: Lt, len: 1 }
        Token { kind: Ident, len: 8 }
        Token { kind: Gt, len: 1 }
        Token { kind: LParen, len: 1 }
        Token { kind: Ident, len: 4 }
        Token { kind: RParen, len: 1 }
        Token { kind: Whitespace, len: 1 }
        "#]];
    check_lexing(src, expect);
}
