use std::path::PathBuf;

#[salsa::input(debug)]
pub struct SourceFile {
    pub path: PathBuf,
}
