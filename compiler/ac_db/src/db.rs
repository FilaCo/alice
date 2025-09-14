#[salsa::db]
#[derive(Default, Clone)]
pub struct AliceDb {
    storage: salsa::Storage<Self>,
}

pub use salsa::Database as AliceDbTrait;

impl AliceDbTrait for AliceDb {}
