use std::{net::TcpStream, sync::Once};

use std::sync::Arc;

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

    config: Arc<Config>,
}

pub(crate) struct SessionOption {
    pub(crate) stream: TcpStream,
    pub(crate) config: Arc<Config>,
}

impl From<SessionOption> for Session {
    fn from(option: SessionOption) -> Self {
        Session {
            conn: Conn::from(option.stream),
            ops: 0,
            create_time: 0,
            last_op_time: 0,
            database: 0,
            quit: false,
            start: Once::new(),
            exit: Once::new(),
            config: Arc::clone(&option.config),
        }
    }
}

impl Session {
    pub(crate) fn start<T: Router>(&self, router: &T) {}
}
