use crate::{db::AliceDbTrait, ir::SourceCode, query::parse::parse};

#[salsa::tracked]
pub fn compile(db: &dyn AliceDbTrait, src: SourceCode) {
    parse(db, src)
}
