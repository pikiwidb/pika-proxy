use crate::proxy::config::Config;
use crate::proxy::delay::DelayExp2;
use crate::proxy::request::Request;
use std::sync::mpsc::Sender;
use std::sync::{
    atomic::{AtomicBool, AtomicI64},
    Once,
};

const STATE_CONNECTED: u64 = 1;
const STATE_DATA_STALE: u32 = 2;

pub struct BackendConn {
    stop: Once,
    addr: String,

    input: Sender<Request>,
    retry: Retry,
    state: AtomicI64,

    closed: AtomicBool,
    config: Config,

    database: i32,
}

struct Retry {
    fails: i32,
    delay: DelayExp2,
}

impl BackendConn {
    pub fn new(addr: String, database: i32, config: Config) -> Self {
        let (tx, _rx) = std::sync::mpsc::channel();
        let retry = Retry {
            fails: 0,
            delay: DelayExp2::new(50, 5000, std::time::Duration::from_millis(1)),
        };

        let bc = BackendConn {
            stop: Once::new(),
            addr,
            input: tx,
            retry,
            state: AtomicI64::new(0),
            closed: AtomicBool::new(false),
            config,
            database,
        };

        // Todo: run bc
        // let mut bc_clone = bc.clone();
        // thread::spawn(move || bc_clone.run(rx));

        bc
    }

    pub fn addr(&self) -> &str {
        &self.addr
    }
}

mod tests {}
