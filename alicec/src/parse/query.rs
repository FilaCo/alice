use crate::{db::AlicecDbTrait, ir::SourceFile};

#[salsa::tracked]
pub fn parse_file(db: &dyn AlicecDbTrait, src: SourceFile) {}
