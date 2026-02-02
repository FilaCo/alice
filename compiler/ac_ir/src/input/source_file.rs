#[salsa::input(debug)]
pub struct SourceFile {
    #[returns(ref)]
    pub content: String,
}
