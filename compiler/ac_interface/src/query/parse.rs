use ac_db::db::AcDbTrait;

#[salsa::tracked]
pub fn parse(db: &dyn AcDbTrait) {}
