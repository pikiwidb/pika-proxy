use std::fmt::{Debug, Display, Formatter};

pub type Result<T> = std::result::Result<T, Error>;
#[derive(Debug, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub enum ErrorKind {
    Unknown, // unknown error
    Redis,   // include redis-rs error
    IO,      // include tokio::io/std::io error
}

pub struct Error {
    kind: ErrorKind,
    inner: anyhow::Error,
}

impl Error {
    pub fn new<E: Into<anyhow::Error>>(kind: ErrorKind, inner: E) -> Self {
        Self {
            kind,
            inner: inner.into(),
        }
    }
    pub fn unknown<E: Into<anyhow::Error>>(inner: E) -> Self {
        Self {
            kind: ErrorKind::Unknown,
            inner: inner.into(),
        }
    }
    pub fn redis<E: Into<anyhow::Error>>(inner: E) -> Self {
        Self {
            kind: ErrorKind::Redis,
            inner: inner.into(),
        }
    }
    pub fn io<E: Into<anyhow::Error>>(inner: E) -> Self {
        Self {
            kind: ErrorKind::IO,
            inner: inner.into(),
        }
    }

    pub fn error_kind(&self) -> &ErrorKind {
        &self.kind
    }
    pub fn is_redis_error(&self) -> bool {
        match self.kind {
            ErrorKind::Redis => true,
            _ => false,
        }
    }
    pub fn is_unknown_error(&self) -> bool {
        match self.kind {
            ErrorKind::Unknown => true,
            _ => false,
        }
    }
    pub fn is_io_error(&self) -> bool {
        match self.kind {
            ErrorKind::IO => true,
            _ => false,
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.kind {
            ErrorKind::Unknown => {
                f.write_fmt(format_args!("unexpected error: {}", self.inner.to_string()))
            }
            ErrorKind::Redis => {
                f.write_fmt(format_args!("redis error: {}", self.inner.to_string()))
            }
            ErrorKind::IO => f.write_fmt(format_args!("io error: {}", self.inner.to_string())),
        }
    }
}
impl Debug for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Error")
            .field(&self.kind)
            .field(&self.inner)
            .finish()
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.inner.source()
    }
}
