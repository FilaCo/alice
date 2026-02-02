use ariadne::Report;

#[salsa::accumulator]
#[derive(Clone)]
pub struct Diagnostic {
    kind: DiagnosticKind,
}

impl Diagnostic {
    pub fn report(self) {
        // let report = Report::build(ReportKind::from(self.kind));
        todo!()
    }
}

#[derive(Clone, Copy)]
pub enum DiagnosticKind {
    Error,
    Warning,
    Notice,
}
