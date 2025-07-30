#![allow(dead_code)]
#![allow(unused)]

pub mod cli;
mod db;
pub mod ir;
pub mod query;

pub mod prelude {
    pub use super::cli::*;
    pub use super::ir::*;
    pub use super::query::*;
}

use salsa::Database as AliceDbTrait;
