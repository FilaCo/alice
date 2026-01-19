#[salsa::tracked(debug)]
pub struct Symbol<'db> {
    #[tracked]
    #[returns(ref)]
    pub value: String,
}
