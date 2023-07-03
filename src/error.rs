pub type Result<T> = std::result::Result<T, NIFileError>;
pub type Error = NIFileError;

#[derive(thiserror::Error, Debug)]
#[non_exhaustive]
pub enum NIFileError {
    #[error("Generic error: {0}")]
    Generic(String),

    #[error(transparent)]
    IO(#[from] std::io::Error),

    #[error("Decompression error")]
    DecompressionError,

    #[error("Incorrect Size Field: expected {expected}, got {got}")]
    IncorrectFrameSize { expected: u64, got: u64 },

    #[error("Encountered Item Terminator")]
    ItemTerminator,

    #[error("Static error: {0}")]
    Static(&'static str),
}
