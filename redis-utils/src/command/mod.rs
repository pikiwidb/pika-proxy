use anyhow::anyhow;
use redis::{parse_redis_value, Value};

use crate::error::{Error, Result};

/// Represents different types of Redis commands
#[derive(Debug, PartialEq)]
pub enum RedisCmd {
    // Basic commands with a single key argument
    Get(String),
    Set(String, String),
    // Other commands...
    // Add more variants as needed for other Redis commands
}

impl RedisCmd {
    // Parse Redis value from bytes
    pub fn parse_redis_value_from_bytes(bytes: &[u8]) -> Result<RedisCmd> {
        let redis_value = parse_redis_value(bytes).map_err(|e| Error::redis(e))?;
        Self::process_redis_value(redis_value)
    }

    // Process a Value and convert it to a RedisCmd
    pub fn process_redis_value(value: Value) -> Result<RedisCmd> {
        match value {
            Value::Bulk(bulk_values) => {
                if bulk_values.is_empty() {
                    return Err(Error::redis(anyhow!("Empty array")));
                }

                // Ensure the first element in the array is a string
                if let Value::Data(command) = &bulk_values[0] {
                    if let Ok(command_str) = std::str::from_utf8(command) {
                        match command_str {
                            "SET" => Self::process_set_command(&bulk_values),
                            "GET" => Self::process_get_command(&bulk_values),
                            _ => Err(Error::redis(anyhow!("Unknown command"))),
                        }
                    } else {
                        Err(Error::redis(anyhow!("Invalid command")))
                    }
                } else {
                    Err(Error::redis(anyhow!("Invalid command")))
                }
            }
            _ => Err(Error::redis(anyhow!("Invalid response"))),
        }
    }

    // Process SET command
    fn process_set_command(bulk_values: &[Value]) -> Result<RedisCmd> {
        // Ensure there are enough arguments for SET command
        if bulk_values.len() != 3 {
            return Err(Error::redis(anyhow!("Invalid SET command")));
        }

        // Extract the key and value from the array
        if let (Value::Data(key), Value::Data(value)) = (&bulk_values[1], &bulk_values[2]) {
            return Ok(RedisCmd::Set(
                String::from_utf8_lossy(key).to_string(),
                String::from_utf8_lossy(value).to_string(),
            ));
        } else {
            return Err(Error::redis(anyhow!("Invalid SET command arguments")));
        }
    }

    // Process GET command
    fn process_get_command(bulk_values: &[Value]) -> Result<RedisCmd> {
        // Ensure there are enough arguments for GET command
        if bulk_values.len() != 2 {
            return Err(Error::redis(anyhow!("Invalid GET command")));
        }

        // Extract the key from the array
        if let Value::Data(key) = &bulk_values[1] {
            return Ok(RedisCmd::Get(String::from_utf8_lossy(key).to_string()));
        } else {
            return Err(Error::redis(anyhow!("Invalid GET command argument")));
        }
    }
}
