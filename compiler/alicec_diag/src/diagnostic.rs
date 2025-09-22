use alicec_db::prelude::AlicecDbTrait;

#[salsa::accumulator]
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
    pub fn unknown_token_start(db: &dyn AlicecDbTrait) -> Self {
        todo!()
    }
}
