use std::ops::Range;

use chumsky::{Parser, input::ValueInput, prelude::end};
use logos::{Lexer, Logos};

use crate::db::AcDbTrait;

#[salsa::tracked]
pub fn parse_file(db: &dyn AcDbTrait) {}

#[salsa::tracked(debug)]
pub struct Cake<'db> {
    #[tracked]
    #[returns(ref)]
    pub statements: Vec<Statement<'db>>,
}

#[salsa::tracked(debug)]
pub struct Statement<'db> {
    pub kind: StatementKind<'db>,
    pub span: Span<'db>,
}

fn parser<'db>(db: &dyn AcDbTrait) -> impl Parser<'db, &'db str, ()> {
    end()
}

use Base::*;
use LiteralKind::*;

#[derive(Logos, Clone, Copy, Debug, PartialEq, Eq)]
#[logos(skip r"[ \t\n\r]+")]
#[logos(skip r"//.*")]
#[logos(subpattern dec_int_lit = r"[0-9][0-9_]*")]
#[logos(subpattern dec_float_lit = r"(?&dec_int_lit)?\.(?&dec_int_lit)?")]
enum Token<'src> {
    /// A block comment, e.g. `/* block comment */`.
    ///
    /// Block comments can be recursive, so a sequence like `/* /* */`
    /// will not be considered terminated and will result in a parsing error.
    #[regex(r"/\*", block_comment)]
    BlockComment(BlockCommentTerminated),

    /// An identifier or keyword, e.g. `ident` or `prop`.
    #[regex(r"[A-Za-z_][A-Za-z0-9_]*")]
    Ident(&'src str),

    #[regex(r"(?&dec_int_lit)", |lex| Literal { kind: Int { base: Dec }, symbol: lex.slice() })]
    #[regex(r"(?&dec_float_lit)", |lex| Literal { kind: Float { base: Dec }, symbol: lex.slice() })]
    Literal(Literal<'src>),

    /// `;`
    #[token(";")]
    Semi,
    /// `,`
    #[token(",")]
    Comma,
    /// `.`
    #[token(".", priority = 3)]
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

    /// `==`
    #[token("==")]
    EqEq,
    /// `!=`
    #[token("!=")]
    Ne,
    /// `<=`
    #[token("<=")]
    Le,
    /// `>=`
    #[token(">=")]
    Ge,

    /// Unknown token, not expected by the lexer, e.g. "№"
    Unknown,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum BlockCommentTerminated {
    No,
    Yes,
}

fn block_comment<'src>(lexer: &mut Lexer<'src, Token<'src>>) -> BlockCommentTerminated {
    use BlockCommentTerminated::*;

    let mut depth = 1usize;
    let mut bump_bytes = 0;
    let mut src = lexer.remainder().chars().peekable();
    while let Some(c) = src.next() {
        bump_bytes += 1;
        match c {
            '/' if src.peek().is_some_and(|v| *v == '*') => {
                src.next();
                bump_bytes += 1;
                depth += 1;
            }
            '*' if src.peek().is_some_and(|v| *v == '/') => {
                src.next();
                bump_bytes += 1;
                depth -= 1;

                if depth == 0 {
                    // This block comment is closed, so for a construction like "/* */ */"
                    // there will be a successfully parsed block comment "/* */"
                    // and " */" will be processed separately.
                    break;
                }
            }
            _ => (),
        }
    }
    lexer.bump(bump_bytes);

    if depth == 0 { Yes } else { No }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Literal<'src> {
    pub kind: LiteralKind,
    pub symbol: &'src str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum LiteralKind {
    Int { base: Base },
    Float { base: Base },
    Rune { terminated: bool },
    Str { terminated: bool },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Base {
    Bin = 2,
    Oct = 8,
    Dec = 10,
    Hex = 16,
}

#[cfg(test)]
mod tests {
    use super::*;
    use expect_test::{Expect, expect};

    fn check_lexing(src: &str, expect: &Expect) {
        let tokens = Token::lexer(src)
            .map(|item| match item {
                Ok(tok) => format!("{:?}\n", tok),
                Err(_) => format!("{:?}\n", Token::Unknown),
            })
            .collect::<String>();
        expect.assert_eq(&tokens);
    }

    #[test]
    fn test_line_comment() {
        // arrange
        let src = r"
// This is a line comment
// Another line comment
//
/  /
";

        let expect = expect![[r"
Slash
Slash
"]];

        check_lexing(src, &expect);
    }

    #[test]
    fn test_block_comment() {
        // arrange
        let src = r"
/* This is a block comment */
/*
Multi
line
block
comment
*/
/*
 * Another
 * multiline
 * block
 * comment
 */
/* */ */
";

        let expect = expect![[r"
BlockComment(Yes)
BlockComment(Yes)
BlockComment(Yes)
BlockComment(Yes)
Star
Slash
"]];

        check_lexing(src, &expect);
    }

    #[test]
    fn test_unterminated_block_comment() {
        let samples = ["/* /* */", "/*"];

        let expects = [
            expect![[r"
BlockComment(No)
"]],
            expect![[r"
BlockComment(No)
"]],
        ];

        for (sample, expect) in samples.into_iter().zip(expects.iter()) {
            check_lexing(sample, expect);
        }
    }

    #[test]
    fn test_ident() {
        let src = r"
foo
_bar
_
foo_bar
1foo123
32f21b
fooBar
FooBar
Foobar
";
        let expect = expect![[r#"
Ident("foo")
Ident("_bar")
Ident("_")
Ident("foo_bar")
Literal(Literal { kind: Int { base: Dec }, symbol: "1" })
Ident("foo123")
Literal(Literal { kind: Int { base: Dec }, symbol: "32" })
Ident("f21b")
Ident("fooBar")
Ident("FooBar")
Ident("Foobar")
"#]];

        check_lexing(src, &expect);
    }

    #[test]
    fn test_int_literal() {
        let src = r"
42
4_2
0600
0_600
170141183460469231731687303715884105727
170_141183_460469_231731_687303_715884_105727
_42
42_
4__2
";
        let expect = expect![[r#"
Literal(Literal { kind: Int { base: Dec }, symbol: "42" })
Literal(Literal { kind: Int { base: Dec }, symbol: "4_2" })
Literal(Literal { kind: Int { base: Dec }, symbol: "0600" })
Literal(Literal { kind: Int { base: Dec }, symbol: "0_600" })
Literal(Literal { kind: Int { base: Dec }, symbol: "170141183460469231731687303715884105727" })
Literal(Literal { kind: Int { base: Dec }, symbol: "170_141183_460469_231731_687303_715884_105727" })
Ident("_42")
Literal(Literal { kind: Int { base: Dec }, symbol: "42_" })
Literal(Literal { kind: Int { base: Dec }, symbol: "4__2" })
"#]];

        check_lexing(src, &expect);
    }

    #[test]
    fn test_float_literal() {
        let src = r"
.
0.
72.40
072.40
2.71828
.25
1_5.
";
        let expect = expect![[r#"
Dot
Literal(Literal { kind: Float { base: Dec }, symbol: "0." })
Literal(Literal { kind: Float { base: Dec }, symbol: "72.40" })
Literal(Literal { kind: Float { base: Dec }, symbol: "072.40" })
Literal(Literal { kind: Float { base: Dec }, symbol: "2.71828" })
Literal(Literal { kind: Float { base: Dec }, symbol: ".25" })
Literal(Literal { kind: Float { base: Dec }, symbol: "1_5." })
"#]];

        check_lexing(src, &expect);
    }
}
