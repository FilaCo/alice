use ac_db::prelude::AliceDbTrait;

#[salsa::accumulator]
#[derive(Debug)]
pub struct Diagnostic {}

impl Diagnostic {
    pub fn unterminated_block_comment(db: &dyn AliceDbTrait) {
        todo!()
    }
}
