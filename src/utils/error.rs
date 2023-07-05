use std::io;
use thiserror::Error;

// todo: 需要完成 Error 的具体设计
#[derive(Debug, Error)]
pub enum PikaProxyError {
    #[error("error on network: {0}")]
    NetWorkError(String),
    #[error("can't not open file: {0}")]
    FailedOpenDB(String),
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
    #[error("Parsing error: {0}")]
    ParseError(#[from] toml::de::Error),
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
