use alicec_db::prelude::AliceDbTrait;

#[salsa::tracked]
pub fn parse(db: &dyn AliceDbTrait) {}
