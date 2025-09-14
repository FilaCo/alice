use crate::{db::AliceDbTrait, ir::SourceCode};

#[salsa::tracked]
pub fn parse(db: &dyn AliceDbTrait, src: SourceCode) {}
