use crate::lexer::Lexer;
use expect_test::{Expect, expect};

fn check_lexing(src: &str, expect: Expect) {
    let actual: String = Lexer::tokenize(src)
        .map(|token| format!("{:?}\n", token))
        .collect();
    expect.assert_eq(&actual)
}

#[test]
fn smoke() {
    check_lexing(
        "/* my source file */ component Point2f32 { x: f32, y: f32 }",
        expect![[r#"
            Token { kind: BlockComment { is_terminated: Yes }, len: 20 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Ident, len: 9 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Ident, len: 9 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: OpenBrace, len: 1 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Ident, len: 1 }
            Token { kind: Unknown, len: 1 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Ident, len: 3 }
            Token { kind: Comma, len: 1 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Ident, len: 1 }
            Token { kind: Unknown, len: 1 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Ident, len: 3 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: CloseBrace, len: 1 }
        "#]],
    )
}

#[test]
fn comment_flavors() {
    check_lexing(
        r"
// line
//// line as well
/// outer doc line
/* block */
/**/
/*** also block */
",
        expect![[r#"
            Token { kind: Whitespace, len: 1 }
            Token { kind: LineComment, len: 7 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: LineComment, len: 17 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: LineComment, len: 18 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: BlockComment { is_terminated: Yes }, len: 11 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: BlockComment { is_terminated: Yes }, len: 4 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: BlockComment { is_terminated: Yes }, len: 18 }
            Token { kind: Whitespace, len: 1 }
        "#]],
    )
}

#[test]
fn nested_block_comments() {
    check_lexing(
        "/* /* */ */\"a\"",
        expect![[r#"
            Token { kind: BlockComment { is_terminated: Yes }, len: 11 }
            Token { kind: Literal { kind: String { is_terminated: Yes }, suffix_start: 3 }, len: 3 }
        "#]],
    )
}

#[test]
fn literal_suffixes() {
    check_lexing(
        r#"
"a"
1234
0b101
0xABC
1.0
1.0e10
2us
"#,
        expect![[r#"
            Token { kind: Whitespace, len: 1 }
            Token { kind: Literal { kind: String { is_terminated: Yes }, suffix_start: 3 }, len: 3 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Literal { kind: Int { base: Decimal, empty_int: No }, suffix_start: 4 }, len: 4 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Literal { kind: Int { base: Binary, empty_int: No }, suffix_start: 5 }, len: 5 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Literal { kind: Int { base: Hexadecimal, empty_int: No }, suffix_start: 5 }, len: 5 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Literal { kind: Float { base: Decimal, empty_exponent: No }, suffix_start: 3 }, len: 3 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Literal { kind: Float { base: Decimal, empty_exponent: No }, suffix_start: 6 }, len: 6 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Literal { kind: Int { base: Decimal, empty_int: No }, suffix_start: 1 }, len: 3 }
            Token { kind: Whitespace, len: 1 }
        "#]],
    )
}
