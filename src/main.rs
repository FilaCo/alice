use ac::prelude::*;
use ariadne::{Report, Source};
use clap::Parser;

fn main() {
    let args = Cli::parse();

    let db = AliceDb::default();
    // TODO: args.input could be a file path.
    let src = SourceProgram::new(&db, args.input);

    compile(&db, src);

    for diag in compile::compile::accumulated::<Diagnostic>(&db, src) {
        let report: Report = diag.into();

        report
            .eprint(Source::from(src.text(&db)))
            .expect("eprint report failed");
    }
}
