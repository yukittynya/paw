use thiserror::Error;

#[derive(Error, Debug)]
pub enum EditorError {
    #[error("Save error")]
    SaveError
}
