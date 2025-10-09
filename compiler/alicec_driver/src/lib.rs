mod cli;

use crate::cli::Cli;
use alicec_db::prelude::AlicecDb;
use alicec_diag::prelude::Diagnostic;
use alicec_error::prelude::{FatalError, FatalErrorMarker};
use alicec_interface::prelude::Config;
use alicec_ir::prelude::SourceFile;
use alicec_query::{compile, parse};
use ariadne::Report;
use clap::Parser;
use std::{panic, process};

/// Exit status code used for successful compilation and help output.
pub const EXIT_SUCCESS: i32 = 0;

/// Exit status code used for compilation failures and invalid flags.
pub const EXIT_FAILURE: i32 = 1;

pub fn catch_with_exit_code(f: impl FnOnce()) -> i32 {
    match catch_fatal_errors(f) {
        Ok(()) => EXIT_SUCCESS,
        _ => EXIT_FAILURE,
    }
}

pub fn catch_fatal_errors<F: FnOnce() -> R, R>(f: F) -> Result<R, FatalError> {
    panic::catch_unwind(panic::AssertUnwindSafe(f)).map_err(|value| {
        if value.is::<FatalErrorMarker>() {
            FatalError
        } else {
            panic::resume_unwind(value);
        }
    })
}

pub fn run_compiler() {
    let args = Cli::parse();
    // let config = Config {
    //     input: args.input,
    //     include_dirs: args.include_directory,
    // };

    let db = AlicecDb::default();
    let source_file = SourceFile::new(&db, "test".into(), "1-(2+3)*3".to_string());
    parse::parse(&db, source_file);
    let diags = parse::parse::accumulated::<Diagnostic>(&db, source_file);
    for diag in diags {
        eprintln!("{:#?}", diag)
    }

    // alicec_interface::run_compiler(config, |ci| {
    //     // let db = &ci.db;
    //     // compile::compile(db, db.source_map());

    //     // let diags = compile::compile::accumulated::<Diagnostic>(db, source_map);
    //     // for diag in diags {
    //     // let report: Report<'_> = diag.into();
    //     // report.eprint(db);
    //     // }
    // });
}

pub fn main() -> ! {
    let exit_code = catch_with_exit_code(run_compiler);
    process::exit(exit_code);
}
