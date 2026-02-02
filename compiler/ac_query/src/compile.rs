use ac_db::db::AcDbTrait;
use ac_ir::input::SourceFile;

#[salsa::tracked]
pub fn compile(db: &dyn AcDbTrait, src: SourceFile) {}
