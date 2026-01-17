use ariadne::{Report, ReportKind};

#[salsa::accumulator]
pub struct Diagnostic {
    kind: DiagnosticKind,
}

impl Diagnostic {
    pub fn report(&self) {
        // let report = Report::build(self.kind.into(), span)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
