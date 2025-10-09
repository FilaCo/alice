pub mod compiler;
pub mod config;

pub mod prelude {
    pub use crate::compiler::*;
    pub use crate::config::*;
    pub use crate::run_compiler;
}

use crate::compiler::Compiler;
use crate::config::Config;

pub fn run_compiler<R>(config: Config, f: impl FnOnce(&Compiler) -> R) -> R {
    todo!()
}
