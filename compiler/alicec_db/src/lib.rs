pub mod db;
pub mod diag;
pub mod span;
pub mod symbol;

pub mod prelude {
    pub use crate::db::*;
    pub use crate::diag::*;
    pub use crate::span::*;
    pub use crate::symbol::*;
}
