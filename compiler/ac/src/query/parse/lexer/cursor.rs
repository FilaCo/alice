use crate::query::parse::lexer::raw_token::LiteralKind;

use super::raw_token::{BlockCommentTerminated, RawToken, RawTokenKind, StringLiteralTerminated};
use std::str::Chars;

#[derive(Debug)]
pub struct Cursor<'src> {
    len_remaining: usize,
    chars: Chars<'src>,
}

impl<'src> Cursor<'src> {}

const COMMA_CHAR: char = ',';
const DOT_CHAR: char = '.';
const EQ_CHAR: char = '=';
const LT_CHAR: char = '<';
const GT_CHAR: char = '>';
const MINUS_CHAR: char = '-';
const PLUS_CHAR: char = '+';
const SLASH_CHAR: char = '/';
const STAR_CHAR: char = '*';
const OPEN_BRACE_CHAR: char = '{';
const CLOSE_BRACE_CHAR: char = '}';
const OPEN_BRACKET_CHAR: char = '[';
const CLOSE_BRACKET_CHAR: char = ']';
const OPEN_PAREN_CHAR: char = '(';
const CLOSE_PAREN_CHAR: char = ')';

const DOUBLE_QUOTE_CHAR: char = '"';

const NEW_LINE_CHAR: char = '\n';
const TAB_CHAR: char = '\t';
const CARRIAGE_RETURN_CHAR: char = '\r';
const SPACE_CHAR: char = ' ';

const EOF_CHAR: char = '\0';
