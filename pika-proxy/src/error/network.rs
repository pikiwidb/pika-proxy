use thiserror::Error;

pub type Result<T> = std::result::Result<T, NetWorkError>;

#[derive(Debug, Error)]
pub enum NetWorkError {
    #[error("error on network: {0}")]
    NetWork(String),
}
