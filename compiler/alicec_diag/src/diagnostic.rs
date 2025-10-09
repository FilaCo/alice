#[salsa::accumulator]
#[derive(Debug)]
pub struct Diagnostic {
    pub kind: DiagnosticKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DiagnosticKind {
    Error,
    Warning,
    Advice,
}

impl Diagnostic {
    pub fn unknown_token_start() -> Self {
        todo!()
    }

    pub fn unexpected_token() -> Self {
        todo!()
    }
}
