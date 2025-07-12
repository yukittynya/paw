use thiserror::Error;

#[derive(Error, Debug)]
pub enum BufferError {
    #[error("Invalid position: ( line: {line}, column: {column} )")]
    InvalidPosition {line: usize, column: usize},

    #[error("Invalid Range")]
    InvalidRange,

    #[error("IO Error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("File not set")]
    FileNotSet,
}
