#[salsa::db]
pub trait AlicecDbTrait: salsa::Database {}

#[derive(Clone, Default)]
#[salsa::db]
pub struct AlicecDb {
    storage: salsa::Storage<Self>,
}

impl salsa::Database for AlicecDb {}

#[salsa::db]
impl AlicecDbTrait for AlicecDb {}
