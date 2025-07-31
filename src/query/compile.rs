use crate::db::AliceDbTrait;
use crate::ir::SourceProgram;
use crate::query::parse;

#[salsa::tracked]
pub fn compile(db: &dyn AliceDbTrait, src: SourceProgram) {
    parse(db, src);
}
