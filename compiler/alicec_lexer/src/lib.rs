mod cursor;
pub mod lexer;
#[cfg(test)]
mod tests;
pub mod token;

pub mod prelude {
    pub use crate::lexer::*;
    pub use crate::token::*;
}
