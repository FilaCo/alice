pub mod cli;
pub mod compile;
pub mod db;
pub mod diag;
mod ir;
mod parse;

pub mod prelude {
    pub use crate::cli::*;
    pub use crate::compile::*;
    pub use crate::db::*;
    pub use crate::diag::*;
}
