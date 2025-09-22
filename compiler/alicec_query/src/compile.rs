use alicec_db::prelude::AlicecDbTrait;

#[salsa::tracked]
pub fn compile(db: &dyn AlicecDbTrait) {}
