#![allow(dead_code)]
#![allow(unused)]

pub mod accumulator;
pub mod cli;
pub mod db;
pub mod ir;
pub mod query;

pub mod prelude {
    pub use super::AliceDbTrait;
    pub use super::accumulator::*;
    pub use super::cli::*;
    pub use super::db::*;
    pub use super::ir::*;
    pub use super::query::*;
}

pub use salsa::Database as AliceDbTrait;
