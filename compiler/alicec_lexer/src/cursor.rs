use std::str::Chars;

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

    /// Moves to the next character.
    pub fn bump(&mut self) -> Option<char> {
        self.chars.next()
    }

    /// Bumps symbols while predicate returns true or until the end of file is reached.
    pub fn bump_while(&mut self, predicate: impl Fn(char) -> bool) {
        while predicate(self.first()) && !self.is_eof() {
            self.bump();
        }
    }

    /// Peeks the next symbol from the input stream without consuming it.
    /// If requested position doesn't exist, `EOF_CHAR` is returned.
    /// However, getting `EOF_CHAR` doesn't always mean actual end of file,
    /// it should be checked with `is_eof` method.
    pub fn first(&self) -> char {
        // `.next()` optimizes better than `.nth(0)`
        self.chars.clone().next().unwrap_or(EOF_CHAR)
    }

    /// Peeks the second symbol from the input stream without consuming it.
    pub fn second(&self) -> char {
        // `.next()` optimizes better than `.nth(1)`
        let mut iter = self.chars.clone();
        iter.next();
        iter.next().unwrap_or(EOF_CHAR)
    }

    /// Returns amount of already bumped symbols.
    pub fn bumped_len(&self) -> usize {
        self.len_remaining - self.chars.as_str().len()
    }

    /// Resets the number of bytes consumed to 0.
    pub fn reset_len_remaining(&mut self) {
        self.len_remaining = self.chars.as_str().len();
    }

    /// Checks if there is nothing more to consume.
    fn is_eof(&self) -> bool {
        self.chars.as_str().is_empty()
    }
}

pub const COMMA_CHAR: char = ',';
pub const DOT_CHAR: char = '.';
pub const EQ_CHAR: char = '=';
pub const LT_CHAR: char = '<';
pub const GT_CHAR: char = '>';
pub const MINUS_CHAR: char = '-';
pub const PLUS_CHAR: char = '+';
pub const SLASH_CHAR: char = '/';
pub const STAR_CHAR: char = '*';
pub const OPEN_BRACE_CHAR: char = '{';
pub const CLOSE_BRACE_CHAR: char = '}';
pub const OPEN_BRACKET_CHAR: char = '[';
pub const CLOSE_BRACKET_CHAR: char = ']';
pub const OPEN_PAREN_CHAR: char = '(';
pub const CLOSE_PAREN_CHAR: char = ')';

pub const UNDERSCORE_CHAR: char = '_';

pub const ZERO_DIGIT_CHAR: char = '0';
pub const NINE_DIGIT_CHAR: char = '9';
pub const A_CHAR: char = 'a';
pub const Z_CHAR: char = 'z';
pub const CAPITAL_A_CHAR: char = 'A';
pub const CAPITAL_Z_CHAR: char = 'Z';
pub const B_CHAR: char = 'b';
pub const CAPITAL_B_CHAR: char = 'B';
pub const O_CHAR: char = 'o';
pub const CAPITAL_O_CHAR: char = 'O';
pub const X_CHAR: char = 'x';
pub const CAPITAL_X_CHAR: char = 'X';
pub const E_CHAR: char = 'e';
pub const CAPITAL_E_CHAR: char = 'E';

pub const DOUBLE_QUOTE_CHAR: char = '"';

pub const BACK_SLASH_CHAR: char = '\\';
pub const NEW_LINE_CHAR: char = '\n';
pub const TAB_CHAR: char = '\t';
pub const CARRIAGE_RETURN_CHAR: char = '\r';
pub const SPACE_CHAR: char = ' ';

pub const EOF_CHAR: char = '\0';

pub fn is_id_start(c: char) -> bool {
    c == UNDERSCORE_CHAR || unicode_xid::UnicodeXID::is_xid_start(c)
}

pub fn is_id_continue(c: char) -> bool {
    unicode_xid::UnicodeXID::is_xid_continue(c)
}

pub fn is_whitespace(c: char) -> bool {
    is_newline(c) || is_tab(c) || is_carriage_return(c) || is_space(c)
}

pub fn is_newline(c: char) -> bool {
    c == NEW_LINE_CHAR
}

pub fn is_tab(c: char) -> bool {
    c == TAB_CHAR
}

pub fn is_carriage_return(c: char) -> bool {
    c == CARRIAGE_RETURN_CHAR
}

pub fn is_space(c: char) -> bool {
    c == SPACE_CHAR
}

pub fn is_decimal_digit(c: char) -> bool {
    (ZERO_DIGIT_CHAR..=NINE_DIGIT_CHAR).contains(&c)
}

pub fn is_hexademical_digit(c: char) -> bool {
    is_decimal_digit(c)
        || (A_CHAR..=Z_CHAR).contains(&c)
        || (CAPITAL_A_CHAR..=CAPITAL_Z_CHAR).contains(&c)
}
