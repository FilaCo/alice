use ariadne::Report;

#[salsa::accumulator]
pub struct Diagnostic {}

impl Diagnostic {
    pub fn report(&self) {}
}

pub enum DiagnosticKind {}
