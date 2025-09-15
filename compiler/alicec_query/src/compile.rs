use alicec_db::prelude::AliceDbTrait;

#[salsa::tracked]
pub fn compile(db: &dyn AliceDbTrait) {}
