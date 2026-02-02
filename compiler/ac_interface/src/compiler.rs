use crate::db::AcDb;
use ac_db::db::AcDbTrait;
use std::path::PathBuf;

#[derive(Clone, Debug)]
pub struct Config {
    pub input: PathBuf,
}

pub fn run_compiler<R>(cfg: Config, f: impl Fn(&dyn AcDbTrait) -> R) -> R {
    f(&AcDb::from(cfg))
}
