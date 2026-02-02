use ac_error::AcResult;
use ac_ir::input::SourceFile;
use salsa::Database;

#[salsa::db]
pub trait AcDbTrait: Database {
    fn input(&self) -> AcResult<SourceFile>;
}
