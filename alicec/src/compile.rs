use crate::db::AlicecDbTrait;

#[salsa::tracked]
pub fn compile<'db>(db: &'db dyn AlicecDbTrait) {}
