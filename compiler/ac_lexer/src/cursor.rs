use std::str::Chars;

#[derive(Debug)]
pub struct Cursor<'src> {
    len_remaining: usize,
    chars: Chars<'src>,
}

impl<'src> Cursor<'src> {
    pub fn new(input: &'src str) -> Self {
        Self {
            chars: input.chars(),
            len_remaining: input.len(),
        }
    }

    pub fn bump(&mut self) -> Option<char> {
        self.chars.next()
    }

    pub fn bump_while(&mut self, predicate: impl Fn(char) -> bool) {
        while predicate(self.peek_first()) && !self.is_eof() {
            self.bump();
        }
    }

    pub fn peek_first(&self) -> char {
        self.chars.clone().next().unwrap_or(EOF_CHAR)
    }

    pub fn peek_second(&self) -> char {
        let mut iter = self.chars.clone();
        iter.next();
        iter.next().unwrap_or(EOF_CHAR)
    }

    pub fn bumped_len(&self) -> usize {
        self.len_remaining - self.chars.as_str().len()
    }

    pub fn reset_len_remaining(&mut self) {
        self.len_remaining = self.chars.as_str().len();
    }

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

pub const DOUBLE_QUOTE_CHAR: char = '"';

pub const NEW_LINE_CHAR: char = '\n';
pub const TAB_CHAR: char = '\t';
pub const CARRIAGE_RETURN_CHAR: char = '\r';
pub const SPACE_CHAR: char = ' ';

pub const EOF_CHAR: char = '\0';

pub fn is_double_quote(c: char) -> bool {
    c == DOUBLE_QUOTE_CHAR
}

pub fn is_id_start(c: char) -> bool {
    c == '_' || unicode_xid::UnicodeXID::is_xid_start(c)
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
