use alicec_db::prelude::AlicecDbTrait;
use alicec_ir::prelude::SourceMap;

#[salsa::tracked]
pub fn compile(db: &dyn AlicecDbTrait, source_map: SourceMap) {}
