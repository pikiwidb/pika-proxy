use std::io;

use redis::error::RedisError;

use crate::error::config::ConfigError;
use crate::error::network::NetWorkError;
use crate::error::server::ServerError;

pub mod config;
pub mod network;
pub mod server;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("unexpected error: {0}")]
    Unexpected(anyhow::Error),

    #[error("error on network: {0}")]
    NetWork(#[from] NetWorkError),

    #[error("IO error: {0}")]
    IO(#[from] io::Error),

    #[error("error on config: {0}")]
    Config(#[from] ConfigError),

    #[error("error on redis: {0}")]
    Redis(#[from] RedisError),

    #[error("error on server: {0}")]
    Server(#[from] ServerError),
}
