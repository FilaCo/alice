use crate::db::AlicecDbTrait;

#[salsa::accumulator]
pub struct Diagnostic;

impl Diagnostic {
    pub fn unknown_token_start(db: &dyn AlicecDbTrait) -> Self {
        todo!()
    }
}
