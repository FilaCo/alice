use clap::{Parser, ValueEnum};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(version)]
pub struct Cli {
    /// Input source file
    pub input: PathBuf,
    /// Kind of output for the compiler to emit.
    /// Each KIND has the default FILE name:
    /// * tokens   - PROJECT_NAME.tok
    /// * ast      - PROJECT_NAME.ast
    #[arg(long, value_name = "KIND[=FILE]", value_parser = parse_emit, verbatim_doc_comment)]
    pub emit: Vec<EmitArg>,
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
