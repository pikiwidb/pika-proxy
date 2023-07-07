use thiserror::Error;
use toml;

pub type Result<T> = std::result::Result<T, ConfigError>;

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("parse toml error: {0}")]
    ParseToml(#[from] toml::de::Error),
}
