use alicec::prelude::*;
use clap::Parser;

fn main() {
    let args = Cli::parse();
    let config = Config::new(
        args.emit.into_iter().map(|arg| (arg.kind, arg.file)),
        args.cake_name,
    );
    let db = AlicecDb::new(config);
    let src = InputFile::new(&db, args.input);
    compile::compile(&db, src);
    let diags = compile::compile::accumulated::<Diagnostic>(&db, src);
    diags.into_iter().for_each(|d| d.report());
}
