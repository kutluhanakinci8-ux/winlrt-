use thiserror::Error;

pub type Result<T> = std::result::Result<T, ArchiveError>;

#[derive(Debug, Error)]
pub enum ArchiveError {
    #[error("IO: {0}")]
    Io(#[from] std::io::Error),
    #[error("ZIP: {0}")]
    Zip(String),
    #[error("henüz desteklenmiyor: {0}")]
    NotYet(&'static str),
}
