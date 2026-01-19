use std::{fmt::Display, ops::Range};

use crate::{db::AcDbTrait, ir::source::SourceFileId};

#[salsa::accumulator]
pub struct Diagnostic {
    pub kind: DiagnosticKind,
    pub msg: String,
    pub span: (Range<usize>, SourceFileId),
}

impl Display for Diagnostic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DiagnosticKind {
    Error,
    Warning,
    Note,
}
