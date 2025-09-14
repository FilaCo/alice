#[salsa::input(debug)]
pub struct SourceCode {
    #[returns(ref)]
    pub contents: String,
}