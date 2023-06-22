use core::time::Duration;
use std::net::TcpStream;

// 这里维护网络连接的读写接口, 暂时不使用自带缓存
pub struct Conn {
    stream: TcpStream,
    // read_timeout: Duration,
}

impl Conn {
    pub fn from_stream(stream: TcpStream) -> Self {
        Conn { stream: stream }
    }
}
