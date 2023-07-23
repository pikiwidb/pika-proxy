use tokio::io::{AsyncReadExt, AsyncWriteExt};

use error::Result;
pub mod error;

pub struct RedisCmd;
pub struct RedisResp;

pub struct RedisRequestReader {}

impl RedisRequestReader {
    pub fn new<R: AsyncReadExt>(reader: R) -> Self {
        Self {}
    }
    pub async fn read_request(&mut self) -> Result<RedisCmd> {
        unimplemented!()
    }
}

pub struct RedisResponder {}

impl RedisResponder {
    pub fn new<W: AsyncWriteExt>(writer: W) -> Self {
        Self {}
    }
    pub async fn send_response(&mut self, resp: &RedisResp) -> Result<()> {
        unimplemented!()
    }
}
