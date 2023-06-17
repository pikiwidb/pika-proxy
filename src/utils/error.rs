use std::io;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PikaProxyError {
    #[error("can't not open file: {0}")]
    FailedOpenDB(String),
    #[error("IO error: {0}")]
    Io(#[source] Box<io::Error>),
    #[error("unexpected error: {0}")]
    UnexpectedError(String),
    #[error("key or value size is invalid")]
    InvalidKeyOrValue,
    #[error("can't decode on empty entry")]
    DecodeOnEmptyEntry,
    #[error("data is truncated")]
    TruncatedData,
    #[error("empty key")]
    EmptyKey,
    #[error("too large key")]
    TooLargeKey,
    #[error("too large value")]
    TooLargeValue,
    #[error("no more data")]
    NoMoreData,
    #[error("expired key")]
    ExpiredKey,
    #[error("merging")]
    AtMerging,
}

pub type Result<T> = std::result::Result<T, PikaProxyError>;
