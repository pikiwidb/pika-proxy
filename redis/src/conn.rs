use tokio::net::TcpStream;

pub struct Conn {
    pub conn: TcpStream,
}

impl Conn {
    pub fn new(conn: TcpStream) -> Self {
        Self { conn }
    }
}

impl From<TcpStream> for Conn {
    fn from(conn: TcpStream) -> Self {
        Self { conn }
    }
}
