use alicec::prelude::*;
use clap::Parser;

fn main() {
    let args = Cli::parse();
    let db = AlicecDb::new(args);
    let src = db.add_source_file(&db.args().input);

    // compile::compile(&db, src);
    // let diags = compile::compile::accumulated::<Diagnostic>(&db, src);
    // diags.into_iter().for_each(|d| d.report(&db));
}
