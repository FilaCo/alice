use ariadne::{Report, ReportKind};

#[salsa::accumulator]
#[derive(Debug)]
pub struct Diagnostic {
    pub kind: DiagnosticKind,
    pub msg: String,
    pub start: usize,
    pub end: usize,
    pub labels: Vec<DiagnosticLabel>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum DiagnosticKind {
    Error,
    Warning,
    Advice,
}

#[derive(Debug)]
pub struct DiagnosticLabel {}

impl Diagnostic {
    pub const fn new(
        kind: DiagnosticKind,
        msg: String,
        start: usize,
        end: usize,
        labels: Vec<DiagnosticLabel>,
    ) -> Self {
        Self {
            kind,
            msg,
            start,
            end,
            labels,
        }
    }
}

impl From<DiagnosticKind> for ReportKind<'_> {
    fn from(value: DiagnosticKind) -> Self {
        match value {
            DiagnosticKind::Error => ReportKind::Error,
            DiagnosticKind::Warning => ReportKind::Warning,
            DiagnosticKind::Advice => ReportKind::Advice,
        }
    }
}

impl From<&Diagnostic> for Report<'_> {
    fn from(value: &Diagnostic) -> Self {
        Report::build(value.kind.into(), value.start..value.end)
            .with_message(&value.msg)
            .finish()
    }
}
