use std::io;
use thiserror::Error;

pub type ProxyResult<T> = Result<T, ProxyError>;

#[derive(Debug, Error)]
pub enum ProxyError {
    #[error("error file: {0}")]
    ConfigFile(String),
    #[error("invalid: {0}")]
    ConfigInvalid(String),
    #[error("{0}")]
    ConfigIO(String),
    #[error("{0}")]
    ConfigParse(String),
    #[error("bad product name: {0}")]
    ProductValidation(String),
    #[error("use of closed proxy")]
    ClosedProxy,
    #[error("error on network: {0}")]
    NetWork(String),
    #[error("can't not open file: {0}")]
    FailedOpenDB(String),
    #[error("IO error: {0}")]
    IO(#[source] Box<io::Error>),
    #[error("unexpected error: {0}")]
    Unexpected(String),
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
    #[error("failed to retrieve next value")]
    NextDelay,
    #[error("listener failed")]
    Listener,
    #[error("router is not online")]
    RouterNotOnline,
    #[error("too many sessions")]
    TooManySessions,
    #[error("too many pipelined requests")]
    TooManyPipelinedRequests,
}

// #[derive(Debug, Error)]
// pub struct TracedError {
//     #[source]
//     cause: Box<dyn std::error::Error + Send + Sync>,
//     backtrace: Backtrace,
// }
//
// impl TracedError {
//     pub fn new<E>(cause: E) -> TracedError
//         where
//             E: 'static + std::error::Error + Send + Sync,
//     {
//         TracedError {
//             cause: Box::new(cause),
//             backtrace: Backtrace::new(),
//         }
//     }
// }
//
// impl fmt::Display for TracedError {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "TracedError: {}", self.cause)
//     }
// }
