use std::{fs::File, io::read_to_string};

use crate::{
    db::AlicecDbTrait,
    ir::{
        input::InputFile,
        source::{SourceFile, SourceFileId},
    },
    query::parse::parse,
};

#[salsa::tracked]
pub fn compile<'db>(db: &'db dyn AlicecDbTrait, input: InputFile) {
    // TODO: diag
    let file = File::open(input.path(db)).expect("unable to open file");
    // TODO: diag
    let contents = read_to_string(file).expect("unable to read file");
    let src = SourceFile::new(db, SourceFileId::new(db, 0), input.path(db), contents);
    let _ = parse(db, src);
}
