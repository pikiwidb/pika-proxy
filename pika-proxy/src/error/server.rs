use thiserror::Error;

pub type Result<T> = std::result::Result<T, ServerError>;

#[derive(Debug, Error)]
pub enum ServerError {
    #[error("can't not open db: {0}")]
    FailedOpenDB(String),
}
