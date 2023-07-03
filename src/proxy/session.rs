use crate::proxy::config::Config;
use crate::proxy::redis::{
    conn::Conn,
    resp::{Resp, RespType},
};
use crate::proxy::request::{Request, RequestChan};
use crate::proxy::router::Router;
use crate::utils::error::ProxyResult;
use serde::Serialize;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use std::{net::TcpStream, sync::Once};

pub struct Session {
    conn: Conn,

    ops: u64,

    create_time: u64,
    last_op_time: u64,

    database: u32,

    quit: bool,

    start: Once,
    exit: Once,

    broken: AtomicBool,
    config: Arc<Config>,

    authorized: bool,
    // stats
}

pub struct SessionOption {
    pub stream: TcpStream,
    pub config: Arc<Config>,
}

impl Session {
    pub fn new(option: SessionOption) -> Self {
        Session {
            conn: Conn::from(option.stream),
            ops: 0,
            create_time: 0,
            last_op_time: 0,
            database: 0,
            quit: false,
            start: Once::new(),
            exit: Once::new(),
            broken: AtomicBool::from(false),
            config: Arc::clone(&option.config),
            authorized: false,
        }
    }

    fn string(&self) -> String {
        "".to_string()
    }

    pub(crate) fn start(&self, r: Router) {}

    fn loop_reader(&self, tasks: &RequestChan, d: &Router) {}
    fn loop_writer(&self, tasks: &RequestChan) {}
}

pub trait Handle {
    fn handle_response(&self, r: &Request) -> ProxyResult<Resp>;
    fn handle_request(&self, r: &Request, d: &Router);
    fn handle_quit(&self, r: &Request);
    fn handle_auth(&self, r: &Request);
    fn handle_select(&self, r: &Request);
    fn handle_request_ping(&self, r: &Request, d: &Router);
    fn handle_request_info(&self, r: &Request, d: &Router);
    fn handle_request_mget(&self, r: &Request, d: &Router);
    fn handle_request_mset(&self, r: &Request, d: &Router);
    fn handle_request_del(&self, r: &Request, d: &Router);
    fn handle_request_exists(&self, r: &Request, d: &Router);
    fn handle_request_slots_info(&self, r: &Request, d: &Router);
    fn handle_request_slots_scan(&self, r: &Request, d: &Router);
    fn handle_request_slots_mapping(&self, r: &Request, d: &Router);
}

impl Handle for Session {
    fn handle_response(&self, request: &Request) -> ProxyResult<Resp> {
        todo!()
    }

    fn handle_request(&self, request: &Request, router: &Router) {
        todo!()
    }

    fn handle_quit(&self, r: &Request) {
        todo!()
    }

    fn handle_auth(&self, r: &Request) {
        todo!()
    }

    fn handle_select(&self, r: &Request) {
        todo!()
    }

    fn handle_request_ping(&self, r: &Request, d: &Router) {
        todo!()
    }

    fn handle_request_info(&self, r: &Request, d: &Router) {
        todo!()
    }

    fn handle_request_mget(&self, r: &Request, d: &Router) {
        todo!()
    }

    fn handle_request_mset(&self, r: &Request, d: &Router) {
        todo!()
    }

    fn handle_request_del(&self, r: &Request, d: &Router) {
        todo!()
    }

    fn handle_request_exists(&self, r: &Request, d: &Router) {
        todo!()
    }

    fn handle_request_slots_info(&self, r: &Request, d: &Router) {
        todo!()
    }

    fn handle_request_slots_scan(&self, r: &Request, d: &Router) {
        todo!()
    }

    fn handle_request_slots_mapping(&self, r: &Request, d: &Router) {
        todo!()
    }
}

pub trait Stats {
    fn incr_op_total(&self);
    fn get_op_stats(&self, opstr: &str);
    fn incr_op_stats(&self, r: &Request, t: RespType);
    fn incr_op_fails(&self, r: &Request);
    fn flush_op_stats(&self, force: bool);
}

impl Stats for Session {
    fn incr_op_total(&self) {
        todo!()
    }

    fn get_op_stats(&self, opstr: &str) {
        todo!()
    }

    fn incr_op_stats(&self, r: &Request, t: RespType) {
        todo!()
    }

    fn incr_op_fails(&self, r: &Request) {
        todo!()
    }

    fn flush_op_stats(&self, force: bool) {
        todo!()
    }
}

mod tests {}
