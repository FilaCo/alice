mod args;

use std::{
    panic::{AssertUnwindSafe, catch_unwind, resume_unwind},
    process::exit,
};

pub fn main() -> ! {
    let exit_code = catch_with_exit_code(|| run_compiler(&args::raw_args()));
    exit(exit_code.to_i32())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum ExitCode {
    Success,
    Failure,
}

impl ExitCode {
    fn to_i32(self) -> i32 {
        match self {
            ExitCode::Success => 0,
            ExitCode::Failure => 1,
        }
    }
}

/// This is the primary entry point for alicec.
fn run_compiler(at_args: &[String]) {
    let at_args = &at_args[1..];
}

fn catch_with_exit_code(f: impl FnOnce()) -> ExitCode {
    match catch_fatal_errors(f) {
        Ok(_) => ExitCode::Success,
        _ => ExitCode::Failure,
    }
}

fn catch_fatal_errors<F: FnOnce() -> R, R>(f: F) -> Result<R, ()> {
    catch_unwind(AssertUnwindSafe(f)).map_err(|value| {
        resume_unwind(value);
    })
}
