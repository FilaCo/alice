use std::path::{Path, PathBuf};

use clap::ValueEnum;

#[derive(Debug, Clone)]
pub struct Config {}

impl Config {
    pub fn new(
        emit_targets: impl IntoIterator<Item = (EmitKind, Option<PathBuf>)>,
        cake_name: Option<String>,
    ) -> Self {
        Self {}
    }
}

/// Kinds of output `alicec` can emit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum EmitKind {
    /// Emit the token stream after lexical analysis.
    Tokens,
    /// Emit the abstract syntax tree after parsing.
    Ast,
}
