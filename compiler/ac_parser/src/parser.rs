use ac_db::db::AcDbTrait;

pub struct Parser<'db> {
    db: &'db dyn AcDbTrait,
}
