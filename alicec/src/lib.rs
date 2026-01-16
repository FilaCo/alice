pub mod accumulator;
pub mod cli;
pub mod config;
pub mod db;
mod ir;
pub mod query;

pub mod prelude {
    pub use crate::accumulator::*;
    pub use crate::cli::*;
    pub use crate::config::*;
    pub use crate::db::*;
    pub use crate::ir::input::*;
    pub use crate::query::*;
}
