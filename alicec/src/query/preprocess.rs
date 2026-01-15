use crate::db::AlicecDbTrait;

#[salsa::tracked]
pub fn preprocess<'db>(db: &'db dyn AlicecDbTrait) {}
