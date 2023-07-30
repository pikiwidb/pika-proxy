pub mod command;
pub mod error;

use anyhow::anyhow;
use async_recursion::async_recursion;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

use crate::error::{Error, Result};
use command::RedisCmd;

/// Reads Redis requests from a reader.
pub struct RedisRequestReader<R: AsyncReadExt + Unpin> {
    reader: R,
    buffer: Vec<u8>,
}

impl<R: AsyncReadExt + Unpin> RedisRequestReader<R> {
    pub fn new(reader: R) -> Self {
        Self {
            reader,
            buffer: Vec::new(),
        }
    }

    // Read a Redis request from the reader.
    pub async fn read_request(&mut self) -> Result<RedisCmd> {
        // Read data from the reader into the buffer.
        self.buffer.clear();
        self.reader
            .read_to_end(&mut self.buffer)
            .await
            .map_err(|e| Error::io(e))?;

        // Process the Redis value to construct the RedisCmd.
        let cmd = RedisCmd::parse_redis_value_from_bytes(&self.buffer)?;

        Ok(cmd)
    }
}

/// Represents different types of Redis responses
pub enum RedisResp {
    // Simple string response
    SimpleString(String),

    // Integer response
    Integer(i64),

    // Bulk string response
    BulkString(Option<String>),

    // Array response
    Array(Vec<RedisResp>),

    // Error response
    Error(String),
}

/// Writes Redis responses to a writer.
pub struct RedisResponder<W: AsyncWriteExt> {
    writer: W,
}

impl<W: AsyncWriteExt + Unpin + Send> RedisResponder<W> {
    pub fn new(writer: W) -> Self {
        Self { writer }
    }

    // Send a Redis response to the writer.
    pub async fn send_response(&mut self, resp: &RedisResp) -> Result<()> {
        match resp {
            RedisResp::SimpleString(s) => self.send_simple_string_response(s).await,
            RedisResp::Integer(i) => self.send_integer_response(*i).await,
            RedisResp::BulkString(data) => self.send_bulk_string_response(data).await,
            RedisResp::Array(values) => self.send_array_response(values).await,
            RedisResp::Error(err) => self.send_error_response(err).await,
        }
    }

    async fn send_simple_string_response(&mut self, s: &str) -> Result<()> {
        let response = format!("+{}\r\n", s);
        self.writer
            .write_all(response.as_bytes())
            .await
            .map_err(|e| Error::redis(e))?;
        Ok(())
    }

    async fn send_integer_response(&mut self, i: i64) -> Result<()> {
        let response = format!(":{}\r\n", i);
        self.writer
            .write_all(response.as_bytes())
            .await
            .map_err(|e| Error::redis(e))?;
        Ok(())
    }

    async fn send_bulk_string_response(&mut self, data: &Option<String>) -> Result<()> {
        match data {
            Some(s) => {
                let response = format!("${}\r\n{}\r\n", s.len(), s);
                self.writer
                    .write_all(response.as_bytes())
                    .await
                    .map_err(|e| Error::redis(e))?;
            }
            None => {
                let response = "$-1\r\n".to_string();
                self.writer
                    .write_all(response.as_bytes())
                    .await
                    .map_err(|e| Error::redis(e))?;
            }
        }
        Ok(())
    }

    #[async_recursion]
    async fn send_array_response(&mut self, values: &[RedisResp]) -> Result<()> {
        let response = format!("*{}\r\n", values.len());
        self.writer
            .write_all(response.as_bytes())
            .await
            .map_err(|e| Error::redis(e))?;

        for value in values {
            match value {
                RedisResp::Array(_) => {
                    // Array cannot contain another Array
                    return Err(Error::redis(anyhow!("Invalid response")));
                }
                _ => self
                    .send_response(value)
                    .await
                    .map_err(|e| Error::redis(e))?,
            }
        }
        Ok(())
    }

    async fn send_error_response(&mut self, err: &str) -> Result<()> {
        let response = format!("-ERR {}\r\n", err);
        self.writer
            .write_all(response.as_bytes())
            .await
            .map_err(|e| Error::redis(e))?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio_test::assert_ok;
    use tokio_test::io::Builder;

    #[test]
    fn test_parse_redis_value() {
        // Test SET command
        let bytes_set = b"*3\r\n$3\r\nSET\r\n$3\r\nkey\r\n$5\r\nvalue\r\n";
        let cmd_set = RedisCmd::parse_redis_value_from_bytes(bytes_set).unwrap();
        assert_eq!(
            cmd_set,
            RedisCmd::Set("key".to_string(), "value".to_string())
        );

        // Test GET command
        let bytes_get = b"*2\r\n$3\r\nGET\r\n$3\r\nkey\r\n";
        let cmd_get = RedisCmd::parse_redis_value_from_bytes(bytes_get).unwrap();
        assert_eq!(cmd_get, RedisCmd::Get("key".to_string()));

        // Test invalid command (unknown command)
        let bytes_unknown = b"*2\r\n$4\r\nTEST\r\n$3\r\nkey\r\n";
        let result_unknown = RedisCmd::parse_redis_value_from_bytes(bytes_unknown);
        assert!(result_unknown.is_err());
        if let Err(err) = result_unknown {
            assert_eq!(
                err.to_string(),
                Error::redis(anyhow!("Unknown command")).to_string()
            );
        } else {
            panic!("Expected Redis error: Unknown command");
        }

        // Test invalid command (empty array)
        let bytes_empty = b"*0\r\n";
        let result_empty = RedisCmd::parse_redis_value_from_bytes(bytes_empty);
        assert!(result_empty.is_err());
        if let Err(err) = result_empty {
            assert_eq!(
                err.to_string(),
                Error::redis(anyhow!("Empty array")).to_string()
            );
        } else {
            panic!("Expected Redis error: Empty array");
        }
    }

    #[test]
    fn test_parse_set_command() {
        let bytes = b"*3\r\n$3\r\nSET\r\n$3\r\nkey\r\n$5\r\nvalue\r\n";
        let cmd = RedisCmd::parse_redis_value_from_bytes(bytes).unwrap();
        assert_eq!(cmd, RedisCmd::Set("key".to_string(), "value".to_string()));
    }

    #[test]
    fn test_parse_get_command() {
        let bytes = b"*2\r\n$3\r\nGET\r\n$3\r\nkey\r\n";
        let cmd = RedisCmd::parse_redis_value_from_bytes(bytes).unwrap();
        assert_eq!(cmd, RedisCmd::Get("key".to_string()));
    }

    #[tokio::test]
    async fn test_read_request() {
        // Test SET command
        let reader_set = Builder::new()
            .read(b"*3\r\n$3\r\nSET\r\n$3\r\nkey\r\n$5\r\nvalue\r\n")
            .build();
        let mut request_reader_set = RedisRequestReader::new(reader_set);
        let result_set = request_reader_set.read_request().await;
        assert_ok!(&result_set);
        assert_eq!(
            result_set.unwrap(),
            RedisCmd::Set("key".to_string(), "value".to_string())
        );

        // Test GET command
        let reader_get = Builder::new()
            .read(b"*2\r\n$3\r\nGET\r\n$3\r\nkey\r\n")
            .build();
        let mut request_reader_get = RedisRequestReader::new(reader_get);
        let result_get = request_reader_get.read_request().await;
        assert_ok!(&result_get);
        assert_eq!(result_get.unwrap(), RedisCmd::Get("key".to_string()));

        // Test invalid command (unknown command)
        let reader_unknown = Builder::new()
            .read(b"*2\r\n$4\r\nTEST\r\n$3\r\nkey\r\n")
            .build();
        let mut request_reader_unknown = RedisRequestReader::new(reader_unknown);
        let result_unknown = request_reader_unknown.read_request().await;
        assert!(result_unknown.is_err());
        assert_eq!(
            result_unknown.unwrap_err().to_string(),
            Error::redis(anyhow!("Unknown command")).to_string()
        );

        // Test invalid command (empty array)
        let reader_empty = Builder::new().read(b"*0\r\n").build();
        let mut request_reader_empty = RedisRequestReader::new(reader_empty);
        let result_empty = request_reader_empty.read_request().await;
        assert!(result_empty.is_err());
        assert_eq!(
            result_empty.unwrap_err().to_string(),
            Error::redis(anyhow!("Empty array")).to_string()
        );

        // Test incomplete request data
        let reader_incomplete = Builder::new().read(b"*3\r\n$3\r\nSET\r\n").build();
        let mut request_reader_incomplete = RedisRequestReader::new(reader_incomplete);
        let result_incomplete = request_reader_incomplete.read_request().await;
        assert!(result_incomplete.is_err());
        assert_eq!(
            result_incomplete.unwrap_err().to_string(),
            Error::redis(anyhow!("unexpected end of file")).to_string()
        );
    }

    #[tokio::test]
    async fn test_send_response() {
        // Test SimpleString response
        let mut writer_simple_string = Vec::new();
        let mut responder_simple_string = RedisResponder::new(&mut writer_simple_string);
        let response_simple_string = RedisResp::SimpleString("OK".to_string());
        let result_simple_string = responder_simple_string
            .send_response(&response_simple_string)
            .await;
        assert_ok!(result_simple_string);
        assert_eq!(writer_simple_string, b"+OK\r\n".to_vec());

        // Test Integer response
        let mut writer_integer = Vec::new();
        let mut responder_integer = RedisResponder::new(&mut writer_integer);
        let response_integer = RedisResp::Integer(42);
        let result_integer = responder_integer.send_response(&response_integer).await;
        assert_ok!(result_integer);
        assert_eq!(writer_integer, b":42\r\n".to_vec());

        // Test BulkString response (Some)
        let mut writer_bulk_string_some = Vec::new();
        let mut responder_bulk_string_some = RedisResponder::new(&mut writer_bulk_string_some);
        let response_bulk_string_some = RedisResp::BulkString(Some("value".to_string()));
        let result_bulk_string_some = responder_bulk_string_some
            .send_response(&response_bulk_string_some)
            .await;
        assert_ok!(result_bulk_string_some);
        assert_eq!(writer_bulk_string_some, b"$5\r\nvalue\r\n".to_vec());

        // Test BulkString response (None)
        let mut writer_bulk_string_none = Vec::new();
        let mut responder_bulk_string_none = RedisResponder::new(&mut writer_bulk_string_none);
        let response_bulk_string_none = RedisResp::BulkString(None);
        let result_bulk_string_none = responder_bulk_string_none
            .send_response(&response_bulk_string_none)
            .await;
        assert_ok!(result_bulk_string_none);
        assert_eq!(writer_bulk_string_none, b"$-1\r\n".to_vec());

        // Test Array response
        let mut writer_array = Vec::new();
        let mut responder_array = RedisResponder::new(&mut writer_array);
        let response_array = RedisResp::Array(vec![
            RedisResp::SimpleString("OK".to_string()),
            RedisResp::Integer(42),
            RedisResp::BulkString(Some("value".to_string())),
        ]);
        let result_array = responder_array.send_response(&response_array).await;
        assert_ok!(result_array);
        assert_eq!(
            writer_array,
            b"*3\r\n+OK\r\n:42\r\n$5\r\nvalue\r\n".to_vec()
        );

        // Test Error response
        let mut writer_error = Vec::new();
        let mut responder_error = RedisResponder::new(&mut writer_error);
        let response_error = RedisResp::Error("Error message".to_string());
        let result_error = responder_error.send_response(&response_error).await;
        assert_ok!(result_error);
        assert_eq!(writer_error, b"-ERR Error message\r\n".to_vec());

        // Test Error response
        let mut writer_error = Vec::new();
        let mut responder_error = RedisResponder::new(&mut writer_error);
        let response_error = RedisResp::Error("Error message".to_string());
        let result_error = responder_error.send_response(&response_error).await;
        assert_ok!(result_error);
        assert_eq!(writer_error, b"-ERR Error message\r\n".to_vec());

        // Test invalid Array response
        let mut writer_invalid_array = Vec::new();
        let mut responder_invalid_array = RedisResponder::new(&mut writer_invalid_array);
        let response_invalid_array = RedisResp::Array(vec![
            RedisResp::SimpleString("OK".to_string()),
            RedisResp::Integer(42),
            RedisResp::BulkString(Some("value".to_string())),
            // This will cause an error since Array cannot contain another Array
            RedisResp::Array(vec![]),
        ]);
        let result_invalid_array = responder_invalid_array
            .send_response(&response_invalid_array)
            .await;
        assert!(result_invalid_array.is_err());
        assert_eq!(
            result_invalid_array.unwrap_err().to_string(),
            Error::redis(anyhow!("Invalid response")).to_string()
        );
    }
}
