#[salsa::interned(debug)]
pub struct Symbol<'db> {
    #[returns(ref)]
    pub value: String,
}
