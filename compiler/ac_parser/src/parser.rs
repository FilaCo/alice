use ac_db::db::AcDbTrait;

use crate::lexer::Lexer;

pub struct Parser<'db> {
    db: &'db dyn AcDbTrait,
    lexer: Lexer<'db>,
}
