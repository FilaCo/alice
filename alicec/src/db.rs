use salsa::{Database, Storage};

#[salsa::db]
pub trait AlicecDbTrait: Database {}

#[derive(Clone)]
#[salsa::db]
pub struct AlicecDb {
    storage: Storage<Self>,
}

impl AlicecDb {
    pub fn new() -> Self {
        Self {
            storage: Storage::default(),
        }
    }
}

impl Database for AlicecDb {}

#[salsa::db]
impl AlicecDbTrait for AlicecDb {}
