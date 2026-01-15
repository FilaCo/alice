pub mod cli;
pub mod db;
pub mod diag;
mod ir;
pub mod query;

pub mod prelude {
    pub use crate::cli::*;
    pub use crate::db::*;
    pub use crate::diag::*;
}
