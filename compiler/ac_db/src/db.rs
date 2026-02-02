use ac_ir::input::SourceFile;
use salsa::Database;

#[salsa::db]
pub trait AcDbTrait: Database {
    fn input(&self) -> SourceFile;
}
