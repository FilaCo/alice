use crate::config::AlicecConfig;
use alicec_db::prelude::AliceDb;

pub struct Alicec {
    db: AliceDb,
}

impl Alicec {
    pub fn new() -> Self {
        Self {
            db: AliceDb::default(),
        }
    }

    pub fn run(&self, config: &AlicecConfig) {}
}

impl Default for Alicec {
    fn default() -> Self {
        Self::new()
    }
}
