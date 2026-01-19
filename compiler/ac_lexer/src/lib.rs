use crate::{cursor::Cursor, token::Token};

pub mod cursor;
pub mod token;

impl Cursor<'_> {
    pub fn advance_token(&mut self) -> Token {
        let Some(first_char) = self.bump() else {
            return Token::eof();
        };

        todo!()
    }
}

pub mod prelude {
    pub use crate::cursor::*;
    pub use crate::token::*;
}
