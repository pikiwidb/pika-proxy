use thiserror::Error;

pub type Result<T> = std::result::Result<T, RedisError>;

#[derive(Debug, Error)]
pub enum RedisError {
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
