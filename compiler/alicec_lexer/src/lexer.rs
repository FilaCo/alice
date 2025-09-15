use crate::{cursor::*, token::*};

#[derive(Debug)]
pub struct Lexer<'src> {
    cursor: Cursor<'src>,
}

impl<'src> Lexer<'src> {
    pub fn new(input: &'src str) -> Self {
        let cursor = Cursor::new(input);
        Self { cursor }
    }

    pub fn cook(&mut self) -> Token {
        let Some(first_char) = self.cursor.bump() else {
            return Token::eof();
        };

        let kind = match first_char {
            SLASH_CHAR => match self.cursor.peek_first() {
                SLASH_CHAR => self.line_comment(),
                STAR_CHAR => self.block_comment(),
                _ => TokenKind::Slash,
            },

            c if is_whitespace(c) => self.whitespace(),
            c if is_id_start(c) => self.ident(),
            c if is_decimal_digit(c) => {
                let literal_kind = self.number_literal(c);
                let suffix_start = self.token_len();
                self.eat_literal_suffix();
                TokenKind::Literal {
                    kind: literal_kind,
                    suffix_start,
                }
            }
            DOUBLE_QUOTE_CHAR => {
                let is_terminated = self.string_literal();
                let suffix_start = self.token_len();
                if matches!(is_terminated, Terminated::Yes) {
                    self.eat_literal_suffix();
                }
                let kind = LiteralKind::String { is_terminated };
                TokenKind::Literal { kind, suffix_start }
            }

            COMMA_CHAR => TokenKind::Comma,
            DOT_CHAR => TokenKind::Dot,
            EQ_CHAR => TokenKind::Eq,
            LT_CHAR => TokenKind::Lt,
            GT_CHAR => TokenKind::Gt,
            MINUS_CHAR => TokenKind::Minus,
            PLUS_CHAR => TokenKind::Plus,
            STAR_CHAR => TokenKind::Star,
            OPEN_BRACE_CHAR => TokenKind::OpenBrace,
            CLOSE_BRACE_CHAR => TokenKind::CloseBrace,
            OPEN_BRACKET_CHAR => TokenKind::OpenBracket,
            CLOSE_BRACKET_CHAR => TokenKind::CloseBracket,
            OPEN_PAREN_CHAR => TokenKind::OpenParen,
            CLOSE_PAREN_CHAR => TokenKind::CloseParen,

            _ => TokenKind::Unknown,
        };

        let res = Token::new(kind, self.token_len());
        self.cursor.reset_len_remaining();

        res
    }

    fn line_comment(&mut self) -> TokenKind {
        self.cursor.bump();

        self.cursor.bump_while(|c| !is_newline(c));
        TokenKind::LineComment
    }

    fn block_comment(&mut self) -> TokenKind {
        self.cursor.bump();

        let mut open_cnt = 1usize;
        while let Some(c) = self.cursor.bump() {
            match c {
                // `/*` branch
                SLASH_CHAR if self.cursor.peek_first() == STAR_CHAR => {
                    self.cursor.bump();
                    open_cnt += 1;
                }
                // `*/` branch
                STAR_CHAR if self.cursor.peek_first() == SLASH_CHAR => {
                    self.cursor.bump();
                    open_cnt -= 1;

                    if open_cnt == 0 {
                        break;
                    }
                }
                _ => (),
            }
        }

        TokenKind::BlockComment {
            is_terminated: if open_cnt == 0 {
                Terminated::Yes
            } else {
                Terminated::No
            },
        }
    }

    fn whitespace(&mut self) -> TokenKind {
        self.cursor.bump_while(is_whitespace);
        TokenKind::Whitespace
    }

    fn ident(&mut self) -> TokenKind {
        self.cursor.bump();
        self.cursor.bump_while(is_id_continue);
        TokenKind::Ident
    }

    fn number_literal(&mut self, first_digit: char) -> LiteralKind {
        let mut base = Base::Decimal;

        if first_digit == ZERO_DIGIT_CHAR {
            match self.cursor.peek_first() {
                B_CHAR | CAPITAL_B_CHAR => {
                    base = Base::Binary;
                    self.cursor.bump();
                    let empty_int = self.eat_decimal_digits();
                    return LiteralKind::Int { base, empty_int };
                }
                O_CHAR | CAPITAL_O_CHAR => {
                    base = Base::Octal;
                    self.cursor.bump();
                    let empty_int = self.eat_decimal_digits();
                    return LiteralKind::Int { base, empty_int };
                }
                X_CHAR | CAPITAL_X_CHAR => {
                    base = Base::Hexadecimal;
                    self.cursor.bump();
                    let empty_int = self.eat_hexademical_digits();
                    return LiteralKind::Int { base, empty_int };
                }
                c if c == UNDERSCORE_CHAR || is_decimal_digit(c) => {
                    self.eat_decimal_digits();
                }
                E_CHAR | CAPITAL_E_CHAR => {}
                _ => {
                    return LiteralKind::Int {
                        base,
                        empty_int: Empty::No,
                    };
                }
            }
        } else {
            self.eat_decimal_digits();
        }

        match self.cursor.peek_first() {
            DOT_CHAR if !is_id_start(self.cursor.peek_second()) => {
                self.cursor.bump();
                let mut empty_exp = Empty::No;
                if self.cursor.peek_first().is_ascii_digit() {
                    self.eat_decimal_digits();
                    match self.cursor.peek_first() {
                        E_CHAR | CAPITAL_E_CHAR => {
                            self.cursor.bump();
                            empty_exp = self.eat_float_exponent();
                        }
                        _ => (),
                    }
                }
                LiteralKind::Float { base, empty_exp }
            }
            E_CHAR | CAPITAL_E_CHAR => {
                self.cursor.bump();
                let empty_exp = self.eat_float_exponent();
                LiteralKind::Float { base, empty_exp }
            }
            _ => LiteralKind::Int {
                base,
                empty_int: Empty::No,
            },
        }
    }

    fn string_literal(&mut self) -> Terminated {
        while let Some(c) = self.cursor.bump() {
            match c {
                DOUBLE_QUOTE_CHAR => return Terminated::Yes,
                BACK_SLASH_CHAR
                    if matches!(
                        self.cursor.peek_first(),
                        DOUBLE_QUOTE_CHAR | BACK_SLASH_CHAR
                    ) =>
                {
                    self.cursor.bump();
                }
                NEW_LINE_CHAR => return Terminated::No,
                _ => (),
            }
        }
        Terminated::No
    }

    fn token_len(&self) -> usize {
        self.cursor.bumped_len()
    }

    fn eat_float_exponent(&mut self) -> Empty {
        if matches!(self.cursor.peek_first(), MINUS_CHAR | PLUS_CHAR) {
            self.cursor.bump();
        }
        self.eat_decimal_digits()
    }

    fn eat_decimal_digits(&mut self) -> Empty {
        let mut empty_int = Empty::Yes;
        loop {
            match self.cursor.peek_first() {
                UNDERSCORE_CHAR => {
                    self.cursor.bump();
                }
                c if is_decimal_digit(c) => {
                    empty_int = Empty::No;
                    self.cursor.bump();
                }
                _ => break,
            }
        }
        empty_int
    }

    fn eat_hexademical_digits(&mut self) -> Empty {
        let mut empty_int = Empty::Yes;
        loop {
            match self.cursor.peek_first() {
                UNDERSCORE_CHAR => {
                    self.cursor.bump();
                }
                c if is_hexademical_digit(c) => {
                    empty_int = Empty::No;
                    self.cursor.bump();
                }
                _ => break,
            }
        }
        empty_int
    }

    fn eat_literal_suffix(&mut self) {
        self.eat_identifier();
    }

    fn eat_identifier(&mut self) {
        if !is_id_start(self.cursor.peek_first()) {
            return;
        }

        self.cursor.bump();

        self.cursor.bump_while(is_id_continue);
    }
}
