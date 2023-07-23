pub use redis::RedisError;

mod redis;

pub type Result<T> = std::result::Result<T, RedisError>;
