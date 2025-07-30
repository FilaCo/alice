#[salsa::input(debug)]
pub struct SourceProgram {
    #[returns(ref)]
    pub text: String,
}
