use crate::{db::AliceDb, diag::Diagnostic, ir::SourceCode, query::compile};
use ariadne::{Report, ReportKind, Source};
use clap::Parser;

pub struct Alice {
    args: Cli,
    db: AliceDb,
}

#[derive(Parser, Debug)]
#[command(version, about)]
struct Cli {
    input: String,
}

impl Alice {
    pub fn new() -> Self {
        let args = Cli::parse();
        let db = AliceDb::default();
        Self { args, db }
    }

    pub fn compile(&self) {
        // TODO: Source management
        let input_path = &self.args.input;
        match std::fs::read_to_string(input_path) {
            Ok(contents) => {
                let src = SourceCode::new(&self.db, contents);
                compile::compile(&self.db, src);

                let diags = compile::compile::accumulated::<Diagnostic>(&self.db, src);
                for diag in diags {
                    let report: Report<'_> = diag.into();

                    report
                        .eprint(Source::from(input_path))
                        .expect("eprint(report) failed")
                }
            }
            Err(err) => Report::build(ReportKind::Error, 0..0)
                .with_message(err.to_string())
                .finish()
                .eprint(Source::from(input_path))
                .expect("eprint(report) failed"),
        }
    }
}

impl Default for Alice {
    fn default() -> Self {
        Self::new()
    }
}
