pub trait AlicecDbTrait: salsa::Database {}

#[salsa::db]
#[derive(Default, Clone)]
pub struct AlicecDb {
    storage: salsa::Storage<Self>,
}

impl salsa::Database for AlicecDb {}

impl AlicecDbTrait for AlicecDb {}
