use ariadne::{Report, ReportKind};

use crate::db::AcDbTrait;

#[salsa::accumulator]
pub struct Diagnostic {
    pub kind: DiagnosticKind,
    pub msg: String,
    pub span: Span,
}

impl Diagnostic {
    pub fn report(&self, db: &dyn AcDbTrait) {
        // Report::build(self.kind.into(), self.span)
        //     .with_message(&self.msg)
        //     .finish()
        //     .eprint(cache);
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DiagnosticKind {
    Error,
    Warning,
}

impl From<DiagnosticKind> for ReportKind<'_> {
    fn from(value: DiagnosticKind) -> Self {
        match value {
            DiagnosticKind::Error => ReportKind::Error,
            DiagnosticKind::Warning => ReportKind::Warning,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    start: usize,
    end: usize,
    fid: usize,
}

impl ariadne::Span for Span {
    type SourceId = usize;

    fn source(&self) -> &Self::SourceId {
        &self.fid
    }

    fn start(&self) -> usize {
        self.start
    }

    fn end(&self) -> usize {
        self.end
    }
}
