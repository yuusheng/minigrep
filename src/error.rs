use glob::PatternError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum GrepError {
    #[error("File not found")]
    FileNotFound,

    #[error("Pattern match error")]
    PatternMatchError(#[from] PatternError),

    #[error("Not found")]
    PatternNotFound,
}
