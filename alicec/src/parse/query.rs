use crate::{
    db::AlicecDbTrait,
    ir::{Ast, SourceFile},
};

#[salsa::tracked]
pub fn parse_file<'db>(db: &'db dyn AlicecDbTrait, src: SourceFile) -> Ast<'db> {
    todo!()
}
