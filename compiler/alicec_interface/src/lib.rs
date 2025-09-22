pub mod config;

pub mod prelude {
    pub use crate::config::*;
    pub use crate::run_compiler;
}

use crate::config::Config;
use alicec_db::prelude::AlicecDbTrait;

pub fn run_compiler<R>(config: Config, f: impl FnOnce(&dyn AlicecDbTrait) -> R) -> R {
    todo!()
}
