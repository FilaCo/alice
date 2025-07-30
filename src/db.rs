#[salsa::db]
#[derive(Default, Clone)]
pub struct AliceDb {
    storage: salsa::Storage<Self>,
}

impl AliceDbTrait for AliceDb {}

pub use salsa::Database as AliceDbTrait;
