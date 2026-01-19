use ac::prelude::*;
use clap::Parser;

fn main() {
    let args = Cli::parse();
    let db = AcDb::new(args);
    let fid = db.add_source_file(&db.args().input);
    let initial = db
        .get_source_file(fid)
        .expect("unable to get initial source file");
    compile::compile(&db, initial);
    let diags = compile::compile::accumulated::<Diagnostic>(&db, initial);
}
