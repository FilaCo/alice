use crate::{
    cursor::Cursor,
    token::{Base, LiteralKind, Token, TokenKind},
};
use Base::*;
use LiteralKind::*;
use TokenKind::*;
use unicode_xid::UnicodeXID;

pub mod cursor;
#[cfg(test)]
mod tests;
pub mod token;

impl Cursor<'_> {
    pub fn advance_token(&mut self) -> Token {
        let Some(first_char) = self.bump() else {
            return Token::eof();
        };

        let kind = match first_char {
            // Slash, comment or block comment.
            '/' => match self.first() {
                '/' => self.line_comment(),
                '*' => self.block_comment(),
                _ => Slash,
            },

            // Whitespace sequence.
            c if is_whitespace(c) => self.whitespace(),

            // Identifier (this should be checked after other variant that can
            // start as identifier).
            c if is_id_start(c) => self.ident(),

            // Numeric literal.
            c @ '0'..='9' => {
                let kind = self.num(c);
                Literal { kind }
            }

            // One-symbol tokens.
            ';' => Semi,
            ',' => Comma,
            '.' => Dot,
            '(' => LParen,
            ')' => RParen,
            '{' => LBrace,
            '}' => RBrace,
            '[' => LBracket,
            ']' => RBracket,
            ':' => Colon,
            '=' => Eq,
            '<' => Lt,
            '>' => Gt,
            '-' => Minus,
            '+' => Plus,
            '*' => Star,
            '%' => Percent,

            // String literal.
            '"' => {
                let terminated = self.str();
                let kind = Str { terminated };
                Literal { kind }
            }

            _ => Unknown,
        };

        let res = Token::new(kind, self.bumped_len());
        self.reset_len_remaining();

        res
    }

    fn line_comment(&mut self) -> TokenKind {
        self.bump();
        self.eat_until(b'\n');

        LineComment
    }

    fn block_comment(&mut self) -> TokenKind {
        self.bump();

        let mut depth = 1usize;
        while let Some(c) = self.bump() {
            match c {
                '/' if self.first() == '*' => {
                    self.bump();
                    depth += 1;
                }
                '*' if self.first() == '/' => {
                    self.bump();
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

        BlockComment {
            terminated: depth == 0,
        }
    }

    fn whitespace(&mut self) -> TokenKind {
        self.eat_while(is_whitespace);

        Whitespace
    }

    fn ident(&mut self) -> TokenKind {
        self.eat_while(is_id_continue);

        Ident
    }

    fn num(&mut self, first_digit: char) -> LiteralKind {
        let mut base = Dec;
        if first_digit == '0' {
            match self.first() {
                'b' | 'B' => {
                    base = Bin;
                    self.bump();
                    if !self.eat_dec_digits() {
                        return Int {
                            base,
                            empty_int: true,
                        };
                    }
                }
                'o' | 'O' => {
                    base = Oct;
                    self.bump();
                    if !self.eat_dec_digits() {
                        return Int {
                            base,
                            empty_int: true,
                        };
                    }
                }
                'x' | 'X' => {
                    base = Hex;
                    self.bump();
                    if !self.eat_hex_digits() {
                        return Int {
                            base,
                            empty_int: true,
                        };
                    }
                }
                // Not a base prefix; consume additional digits.
                '0'..='9' | '_' => {
                    self.eat_dec_digits();
                }
                // Just a 0.
                _ => {
                    return Int {
                        base,
                        empty_int: false,
                    };
                }
            }
        } else {
            // No base prefix, parse number in the usual way.
            self.eat_dec_digits();
        }

        Int {
            base,
            empty_int: false,
        }
    }

    fn str(&mut self) -> bool {
        while let Some(c) = self.bump() {
            match c {
                '"' => {
                    return true;
                }
                '\\' if self.first() == '\\' || self.first() == '"' => {
                    // Bump again to skip escaped character.
                    self.bump();
                }
                _ => (),
            }
        }
        // End of file reached.
        false
    }

    fn eat_dec_digits(&mut self) -> bool {
        let mut has_digits = false;
        loop {
            match self.first() {
                '_' => {
                    self.bump();
                }
                '0'..='9' => {
                    has_digits = true;
                    self.bump();
                }
                _ => break,
            }
        }
        has_digits
    }

    fn eat_hex_digits(&mut self) -> bool {
        let mut has_digits = false;
        loop {
            match self.first() {
                '_' => {
                    self.bump();
                }
                '0'..='9' | 'a'..='f' | 'A'..='F' => {
                    has_digits = true;
                    self.bump();
                }
                _ => break,
            }
        }
        has_digits
    }
}

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

/// True if `c` is valid as a first character of an identifier.
fn is_id_start(c: char) -> bool {
    // This is XID_Start OR '_' (which formally is not a XID_Start).
    c == '_' || UnicodeXID::is_xid_start(c)
}

/// True if `c` is valid as a non-first character of an identifier.
fn is_id_continue(c: char) -> bool {
    UnicodeXID::is_xid_continue(c)
}

pub mod prelude {
    pub use crate::cursor::*;
    pub use crate::token::*;
}
