use thiserror::Error;

pub type AcResult<T> = Result<T, AcError>;

#[derive(Error, Debug)]
pub enum AcError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}
