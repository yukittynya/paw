use thiserror::Error;

#[derive(Error, Debug)]
pub enum BufferError {
    #[error("Invalid position: ( line: {line}, column: {column} )")]
    InvalidPosition {line: usize, column: usize},

    #[error("Invalid Range")]
    InvalidRange
}
