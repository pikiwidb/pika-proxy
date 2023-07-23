use std::fmt::{Debug, Display, Formatter};

pub type Result<T> = std::result::Result<T, Error>;
#[derive(Debug, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub enum ErrorKind {
    Unknown,    // unknown error
    Network,    // include tokio/std network error
    Initialize, // error when initialize, include config file read/parse error, etc
    Protocol,   // error when parse or write redis cmd or response data
    Proxy,      // error when proxy data between client and server
    Server,     // server internal error, include registry communication error, etc
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
    pub fn network<E: Into<anyhow::Error>>(inner: E) -> Self {
        Self {
            kind: ErrorKind::Network,
            inner: inner.into(),
        }
    }
    pub fn initialize<E: Into<anyhow::Error>>(inner: E) -> Self {
        Self {
            kind: ErrorKind::Initialize,
            inner: inner.into(),
        }
    }
    pub fn protocol<E: Into<anyhow::Error>>(inner: E) -> Self {
        Self {
            kind: ErrorKind::Protocol,
            inner: inner.into(),
        }
    }
    pub fn proxy<E: Into<anyhow::Error>>(inner: E) -> Self {
        Self {
            kind: ErrorKind::Proxy,
            inner: inner.into(),
        }
    }
    pub fn server<E: Into<anyhow::Error>>(inner: E) -> Self {
        Self {
            kind: ErrorKind::Server,
            inner: inner.into(),
        }
    }

    pub fn error_kind(&self) -> &ErrorKind {
        &self.kind
    }
    pub fn is_network_error(&self) -> bool {
        match self.kind {
            ErrorKind::Network => true,
            _ => false,
        }
    }
    pub fn is_unknown_error(&self) -> bool {
        match self.kind {
            ErrorKind::Unknown => true,
            _ => false,
        }
    }
    pub fn is_initialize_error(&self) -> bool {
        match self.kind {
            ErrorKind::Initialize => true,
            _ => false,
        }
    }
    pub fn is_protocol_error(&self) -> bool {
        match self.kind {
            ErrorKind::Protocol => true,
            _ => false,
        }
    }
    pub fn is_proxy_error(&self) -> bool {
        match self.kind {
            ErrorKind::Proxy => true,
            _ => false,
        }
    }
    pub fn is_server_error(&self) -> bool {
        match self.kind {
            ErrorKind::Server => true,
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
            ErrorKind::Network => {
                f.write_fmt(format_args!("network error: {}", self.inner.to_string()))
            }
            ErrorKind::Initialize => {
                f.write_fmt(format_args!("initialize error: {}", self.inner.to_string()))
            }
            ErrorKind::Protocol => {
                f.write_fmt(format_args!("protocol error: {}", self.inner.to_string()))
            }
            ErrorKind::Proxy => {
                f.write_fmt(format_args!("proxy error: {}", self.inner.to_string()))
            }
            ErrorKind::Server => {
                f.write_fmt(format_args!("server error: {}", self.inner.to_string()))
            }
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
