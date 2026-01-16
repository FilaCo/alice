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

/// Kinds of output the alicec can emit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum EmitKind {
    /// Emit the token stream after lexical analysis.
    Tokens,
    /// Emit the abstract syntax tree after parsing.
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
    let mut parts = s.split('=');
    let kind = parts
        .next()
        .ok_or(String::from("KIND is missing"))
        .and_then(|kind| EmitKind::from_str(kind, true))?;
    let file = parts
        .next()
        .map(|s| s.trim())
        .map(|s| {
            if s.is_empty() {
                Err(String::from("FILE is empty"))
            } else {
                Ok(PathBuf::from(s))
            }
        })
        .transpose()?;

    if parts.count() > 0 {
        return Err(String::from(
            "too many '=' characters, expected format KIND[=FILE]",
        ));
    }

    Ok(EmitArg { kind, file })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_emit() {
        let cases = [
            ("tokens", EmitKind::Tokens),
            ("Tokens", EmitKind::Tokens),
            ("TOKENS", EmitKind::Tokens),
            ("ast", EmitKind::Ast),
            ("Ast", EmitKind::Ast),
            ("AST", EmitKind::Ast),
        ];

        for (input, expected_kind) in cases {
            let result = parse_emit(input);
            let emit_arg = result.expect("parse_emit failed");
            assert_eq!(emit_arg.kind, expected_kind);
            assert_eq!(emit_arg.file, None);
        }
    }

    #[test]
    fn test_parse_emit_with_file() {
        let cases = [
            ("tokens=output.tok", EmitKind::Tokens, "output.tok"),
            ("ast=debug.ast", EmitKind::Ast, "debug.ast"),
            ("Tokens=MYTOKENS.TOK", EmitKind::Tokens, "MYTOKENS.TOK"),
        ];

        for (input, expected_kind, expected_file) in cases {
            let result = parse_emit(input);
            let emit_arg = result.expect("parse_emit failed");
            assert_eq!(emit_arg.kind, expected_kind);
            assert_eq!(emit_arg.file, Some(PathBuf::from(expected_file)),);
        }
    }

    #[test]
    fn test_parse_emit_errors() {
        // Invalid emission kind
        assert!(parse_emit("invalid").is_err());
        assert!(parse_emit("tokens=file.tok=extra").is_err());

        // Empty file path
        assert!(parse_emit("tokens=").is_err());
        assert!(parse_emit("ast=   ").is_err());
    }

    #[test]
    fn test_emit_arg_equality() {
        let arg1 = EmitArg {
            kind: EmitKind::Tokens,
            file: Some(PathBuf::from("file1.tok")),
        };

        let arg2 = EmitArg {
            kind: EmitKind::Tokens,
            file: Some(PathBuf::from("file2.tok")),
        };

        let arg3 = EmitArg {
            kind: EmitKind::Ast,
            file: Some(PathBuf::from("file.ast")),
        };

        // Same kind, different files should be equal
        assert_eq!(arg1, arg2);

        // Different kinds should not be equal
        assert_ne!(arg1, arg3);
        assert_ne!(arg2, arg3);

        // Reflexivity
        assert_eq!(arg1, arg1);
        assert_eq!(arg2, arg2);
        assert_eq!(arg3, arg3);
    }
}
