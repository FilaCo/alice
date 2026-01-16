use crate::{db::AlicecDbTrait, ir::input::SourceFile};

#[salsa::tracked]
pub fn compile<'db>(db: &'db dyn AlicecDbTrait, src: SourceFile) {}
