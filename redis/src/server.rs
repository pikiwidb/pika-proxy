use std::{sync::atomic::AtomicU64, time::Duration};

use anyhow::{bail, Result};
use tokio::net::{TcpListener, ToSocketAddrs};

use crate::conn::Conn;

#[derive(Debug)]
pub struct Server {
    pub listener: TcpListener,
    pub timeout: Duration,
    pub current_conns: AtomicU64,
    pub max_conns: u64,
}

#[derive(Debug)]
pub struct ServerOptions<A: ToSocketAddrs> {
    pub timeout: Duration,
    pub max_conns: u64,
    pub addr: A,
}

impl Server {
    pub async fn new<A: ToSocketAddrs>(option: ServerOptions<A>) -> Result<Self> {
        let listener = TcpListener::bind(option.addr).await?;
        Ok(Server {
            listener,
            timeout: option.timeout,
            max_conns: option.max_conns,
            current_conns: AtomicU64::new(0),
        })
    }

    pub async fn accept(&mut self) -> Result<Conn> {
        let current_conns = self
            .current_conns
            .load(std::sync::atomic::Ordering::Relaxed);
        if current_conns >= self.max_conns {
            bail!("attempt new conn while reached max connection")
        }
        let (t, addr) = self.listener.accept().await?;
        tracing::debug!("new client connection from {}", addr);
        self.current_conns
            .fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        Ok(t.into())
    }
}
