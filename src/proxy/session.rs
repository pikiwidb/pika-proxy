use std::{net::TcpStream, os::unix::raw::time_t, sync::Once};

use super::{config::Config, redis::conn::Conn, router::Router};

// session 对应的是一个 client 的连接
pub struct Session {
    conn: Conn,

    ops: u64,
    create_time: u64,
    last_op_time: u64,

    database: u32,
    quit: bool,

    start: Once,
    exit: Once,

    config: &'static Config,
}

impl Session {
    fn from_config(stream: TcpStream, config: &'static Config) -> Self {
        Self {
            conn: Conn::from_stream(stream),
            ops: 0,
            create_time: 0,
            last_op_time: 0,
            database: 0,
            quit: false,
            start: Once::new(),
            exit: Once::new(),
            config: config,
        }
    }

    fn start<T: Router>(&self, router: &T) {}
}
