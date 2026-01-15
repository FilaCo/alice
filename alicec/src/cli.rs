use clap::{ArgAction, ColorChoice, Parser, ValueEnum};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(version)]
pub struct Cli {
    /// Input source files
    pub input: Vec<PathBuf>,
    /// Write output to FILENAME
    #[arg(short, group = "out", value_name = "FILENAME")]
    pub output: Option<PathBuf>,
    /// Add directory to include search path
    #[arg(short = 'I', long, value_name = "DIR")]
    pub include_directory: Vec<PathBuf>,
    /// Kind of output for the compiler to emit.
    /// Each KIND has the default FILE name:
    /// * tokens - PROJECT_NAME.tok
    /// * ast    - PROJECT_NAME.ast
    #[arg(long, value_name = "KIND[=FILE]", value_parser = parse_emit, verbatim_doc_comment)]
    pub emit: Vec<EmitArg>,
    /// Use verbose output
    #[arg(short, long, action = ArgAction::Count)]
    pub verbose: u8,
    /// Coloring
    #[arg(long, value_name = "WHEN", default_value_t = ColorChoice::Auto)]
    pub color: ColorChoice,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum EmitKind {
    Tokens,
    Ast,
}

#[derive(Debug, Clone, Eq)]
pub struct EmitArg {
    pub kind: EmitKind,
    pub file: Option<PathBuf>,
}

impl PartialEq for EmitArg {
    fn eq(&self, other: &Self) -> bool {
        self.kind == other.kind
    }
}

fn parse_emit(s: &str) -> Result<EmitArg, String> {
    if let Some((kind, file)) = s.split_once('=') {
        let kind = EmitKind::from_str(kind, true)?;
        let file = Some(PathBuf::from(file));

        Ok(EmitArg { kind, file })
    } else {
        let kind = EmitKind::from_str(s, true)?;
        Ok(EmitArg { kind, file: None })
    }
}
