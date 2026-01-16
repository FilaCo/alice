use salsa::{Database, Storage};

use crate::config::Config;

#[salsa::db]
pub trait AlicecDbTrait: Database {}

#[derive(Clone)]
#[salsa::db]
pub struct AlicecDb {
    storage: Storage<Self>,
    config: Config,
}

impl AlicecDb {
    pub fn new(config: Config) -> Self {
        Self {
            storage: Storage::default(),
            config,
        }
    }
}

impl Database for AlicecDb {}

#[salsa::db]
impl AlicecDbTrait for AlicecDb {}
