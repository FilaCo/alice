use logos::{Lexer, Logos};

use crate::db::AcDbTrait;

#[salsa::tracked]
pub fn parse(db: &dyn AcDbTrait) {}

#[derive(Logos, Clone, Copy, Debug, PartialEq, Eq)]
#[logos(skip r"[ \t\n\r]+")]
#[logos(skip r"//.*")]
enum Token<'src> {
    /// An identifier or keyword, e.g. `ident` or `prop`.
    #[regex(r"_|([A-Za-z][A-Za-z0-9_]*)")]
    Ident(&'src str),

    #[regex(r"/\*", block_comment)]
    BlockComment(BlockCommentTerminated),

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

#[cfg(test)]
mod tests {
    use super::*;
    use expect_test::{Expect, expect};

    fn check_lexing(src: &str, expect: Expect) {
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

        check_lexing(src, expect);
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
/*
";

        let expect = expect![[r"
BlockComment(Yes)
BlockComment(Yes)
BlockComment(Yes)
BlockComment(Yes)
Star
Slash
BlockComment(No)
"]];

        check_lexing(src, expect);
    }
}
