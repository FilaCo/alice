use std::path::PathBuf;

#[salsa::input(debug)]
pub struct InputFile {
    pub path: PathBuf,
}
