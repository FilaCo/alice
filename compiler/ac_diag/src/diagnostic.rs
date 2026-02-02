#[salsa::accumulator]
#[derive(Clone, Debug)]
pub struct Diagnostic {
    pub kind: DiagnosticKind,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DiagnosticKind {
    Error,
    Warning,
    Notice,
}
