use alicec::prelude::*;
use clap::Parser;

fn main() {
    let args = Cli::parse();
    let db = AlicecDb::new();
    let src = SourceFile::new(&db, args.input);
    compile::compile(&db, src);
    let diags = compile::compile::accumulated::<Diagnostic>(&db, src);
    diags.iter().for_each(|&d| d.report());
}
