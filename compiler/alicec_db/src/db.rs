#[salsa::db]
pub trait AlicecDbTrait: salsa::Database {}

#[salsa::db]
#[derive(Default, Clone)]
pub struct AlicecDb {
    storage: salsa::Storage<Self>,
}

impl salsa::Database for AlicecDb {}

#[salsa::db]
impl AlicecDbTrait for AlicecDb {}
