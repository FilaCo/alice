use crate::{db::AcDbTrait, ir::source::SourceFile};

#[salsa::tracked]
pub fn compile<'db>(db: &'db dyn AcDbTrait, src: SourceFile) {
    // let _ = parse_file(db, src);
}
