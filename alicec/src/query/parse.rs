use crate::{db::AlicecDbTrait, ir::ast::Ast};

#[salsa::tracked]
pub fn parse<'db>(db: &'db dyn AlicecDbTrait) -> Ast<'db> {
    todo!()
}
