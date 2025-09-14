#[salsa::input(debug)]
pub struct SourceCode {
    #[returns(ref)]
    pub contents: String,
}

#[salsa::tracked(debug)]
pub struct Ast<'db> {}
