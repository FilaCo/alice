use ariadne::{Report, ReportKind};

#[salsa::accumulator]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Diagnostic {
    pub kind: Kind,
    pub msg: String,
}

impl Diagnostic {
    pub fn new(kind: Kind, msg: String) -> Self {
        Self { kind, msg }
    }
}

impl<'a> From<&'a Diagnostic> for Report<'a> {
    fn from(value: &'a Diagnostic) -> Self {
        Report::build(value.kind.into(), 0..0)
            .with_message(value.msg.as_str())
            .finish()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Kind {
    Error,
    Warning,
    Advice,
}

impl From<Kind> for ReportKind<'_> {
    fn from(value: Kind) -> Self {
        match value {
            Kind::Error => ReportKind::Error,
            Kind::Warning => ReportKind::Warning,
            Kind::Advice => ReportKind::Advice,
        }
    }
}
