use crate::db::AlicecDbTrait;

#[salsa::tracked]
pub fn parse<'db>(db: &'db dyn AlicecDbTrait) {}
