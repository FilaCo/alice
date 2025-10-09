use alicec_db::prelude::AlicecDbTrait;
use alicec_ir::prelude::SourceFile;
use alicec_parser::prelude::Parser;

#[salsa::tracked]
pub fn parse(db: &dyn AlicecDbTrait, source_file: SourceFile) {
    let mut parser = Parser::new(db, source_file.contents(db));
    println!("{:#?}", parser.parse_expr());
}
