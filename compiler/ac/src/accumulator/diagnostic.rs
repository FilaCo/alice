use std::{fmt::Display, ops::Range};

use crate::{db::AcDbTrait, ir::source::SourceFileId};

#[salsa::accumulator]
pub struct Diagnostic {
    pub msg: String,
    pub span: Span,
}

pub type Span = (SourceFileId, Range<usize>);

impl Diagnostic {
    pub fn eprint(&self, db: &dyn AcDbTrait) {}
}
