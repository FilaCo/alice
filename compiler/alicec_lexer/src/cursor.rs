use std::str::Chars;

use crate::token::{LitKind, Token, TokenKind};

use LitKind::*;
use TokenKind::*;

/// Peekable iterator over a char sequence.
///
/// Next characters can be peeked via `first` method,
/// and position can be shifted forward via `bump` method.
#[derive(Debug)]
pub struct Cursor<'src> {
    len_remaining: usize,
    /// Iterator over chars. Slightly faster than a &str.
    chars: Chars<'src>,
}

impl<'src> Cursor<'src> {
    pub fn new(input: &'src str) -> Self {
        Self {
            chars: input.chars(),
            len_remaining: input.len(),
        }
    }

    pub fn tokenize(input: &'src str) -> impl Iterator<Item = Token> {
        let mut cursor = Self::new(input);

        std::iter::from_fn(move || {
            let lexeme = cursor.bump_token();

            if lexeme.kind != Eof {
                Some(lexeme)
            } else {
                None
            }
        })
    }

    pub fn bump_token(&mut self) -> Token {
        let Some(first_char) = self.bump() else {
            return Token { kind: Eof, len: 0 };
        };

        let kind = match first_char {
            c if is_whitespace(c) => self.bump_whitespace(),
            c @ '0'..='9' => {
                let lit_kind = self.bump_num_lit(c);
                Lit(lit_kind)
            }

            '-' => Minus,
            '+' => Plus,
            '/' => Slash,
            '*' => Star,

            '(' => LParen,
            ')' => RParen,

            _ => Unknown,
        };

        let res = Token {
            kind,
            len: self.bumped_len(),
        };
        self.reset_len_remaining();

        res
    }

    fn bump_whitespace(&mut self) -> TokenKind {
        self.bump_while(is_whitespace);
        Whitespace
    }

    fn bump_num_lit(&mut self, first_digit: char) -> LitKind {
        if first_digit == '0' {
            match self.first() {
                '0'..='9' => {
                    self.bump_digits();
                }
                _ => {
                    return Int;
                }
            }
        } else {
            self.bump_digits();
        }

        match self.first() {
            '.' => {
                self.bump();
                if let '0'..='9' = self.first() {
                    self.bump_digits();
                }

                Float
            }
            _ => Int,
        }
    }

    fn bump_digits(&mut self) {
        while let '0'..='9' = self.first() {
            self.bump();
        }
    }

    /// Moves to the next character.
    fn bump(&mut self) -> Option<char> {
        self.chars.next()
    }

    /// Bumps chars while predicate returns true or until the end of file is reached.
    fn bump_while(&mut self, predicate: impl Fn(char) -> bool) {
        while predicate(self.first()) && !self.is_at_eof() {
            self.bump();
        }
    }

    /// Peeks the next symbol from the input stream without consuming it.
    /// If requested position doesn't exist, `EOF_CHAR` is returned.
    /// However, getting `EOF_CHAR` doesn't always mean actual end of file,
    /// it should be checked with `is_eof` method.
    fn first(&self) -> char {
        // `.next()` optimizes better than `.nth(0)`
        self.chars.clone().next().unwrap_or(EOF_CHAR)
    }

    /// Returns amount of already bumped symbols.
    fn bumped_len(&self) -> usize {
        self.len_remaining - self.chars.as_str().len()
    }

    /// Resets the number of bytes consumed to 0.
    fn reset_len_remaining(&mut self) {
        self.len_remaining = self.chars.as_str().len();
    }

    /// Checks if there is nothing more to consume.
    fn is_at_eof(&self) -> bool {
        self.chars.as_str().is_empty()
    }
}

const EOF_CHAR: char = '\0';

fn is_whitespace(c: char) -> bool {
    matches!(
        c,
        // End-of-line characters
        | '\u{000A}' // line feed (\n)
        | '\u{000B}' // vertical tab
        | '\u{000C}' // form feed
        | '\u{000D}' // carriage return (\r)
        | '\u{0085}' // next line (from latin1)
        | '\u{2028}' // LINE SEPARATOR
        | '\u{2029}' // PARAGRAPH SEPARATOR

        // `Default_Ignorable_Code_Point` characters
        | '\u{200E}' // LEFT-TO-RIGHT MARK
        | '\u{200F}' // RIGHT-TO-LEFT MARK

        // Horizontal space characters
        | '\u{0009}' // tab (\t)
        | '\u{0020}' // space
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use expect_test::{Expect, expect};

    fn check_lexing(src: &str, expect: Expect) {
        let actual: String = Cursor::tokenize(src)
            .map(|token| format!("{:?}\n", token))
            .collect();
        expect.assert_eq(&actual)
    }

    #[test]
    fn smoke() {
        check_lexing(
            r"
1 + 2 * 3 - 10 * (123 + 321) / 31.12321
",
            expect![[r"
Token { kind: Whitespace, len: 1 }
Token { kind: Lit(Int), len: 1 }
Token { kind: Whitespace, len: 1 }
Token { kind: Plus, len: 1 }
Token { kind: Whitespace, len: 1 }
Token { kind: Lit(Int), len: 1 }
Token { kind: Whitespace, len: 1 }
Token { kind: Star, len: 1 }
Token { kind: Whitespace, len: 1 }
Token { kind: Lit(Int), len: 1 }
Token { kind: Whitespace, len: 1 }
Token { kind: Minus, len: 1 }
Token { kind: Whitespace, len: 1 }
Token { kind: Lit(Int), len: 2 }
Token { kind: Whitespace, len: 1 }
Token { kind: Star, len: 1 }
Token { kind: Whitespace, len: 1 }
Token { kind: LParen, len: 1 }
Token { kind: Lit(Int), len: 3 }
Token { kind: Whitespace, len: 1 }
Token { kind: Plus, len: 1 }
Token { kind: Whitespace, len: 1 }
Token { kind: Lit(Int), len: 3 }
Token { kind: RParen, len: 1 }
Token { kind: Whitespace, len: 1 }
Token { kind: Slash, len: 1 }
Token { kind: Whitespace, len: 1 }
Token { kind: Lit(Float), len: 8 }
Token { kind: Whitespace, len: 1 }
"]],
        );
    }
}
