use core::time::Duration;
use std::net::TcpStream;

// 这里维护网络连接的读写接口, 暂时不使用自带缓存
pub struct Conn {
    stream: TcpStream,
    // read_timeout: Duration,
}

impl From<TcpStream> for Conn {
    fn from(stream: TcpStream) -> Self {
        Conn { stream: stream }
    }
}

impl Conn {
    pub(crate) fn decode(&self) {}
}
