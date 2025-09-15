mod cursor;
pub mod lexer;
pub mod token;

pub mod prelude {
    pub use crate::lexer::*;
    pub use crate::token::*;
}
