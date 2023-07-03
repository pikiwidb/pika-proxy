use super::config::Config;
use crate::models::proxy::Proxy as ProxyModel;
use crate::models::store;
use crate::proxy::delay::{Delay, DelayExp2};
use crate::proxy::metrics::Metrics;
use crate::utils::error::{ProxyError, ProxyResult};
use core::str::FromStr;
use log::{info, trace, warn};
use std::error::Error;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt, BufStream},
    net::{TcpListener, TcpStream},
    sync::{RwLock, RwLockReadGuard, RwLockWriteGuard},
};

#[derive(Debug)]
pub struct Proxy {
    proxy: Arc<RwLock<RawProxy>>,
}

#[derive(Debug)]
pub struct RawProxy {
    xauth: String,
    model: ProxyModel,

    online: bool,
    closed: bool,

    lproxy_addr: SocketAddr,
    ladmin_addr: SocketAddr,

    config: Arc<Config>,
}

pub(crate) struct ProxyOptions {
    pub(crate) config_path: String,
}

impl Proxy {
    pub(crate) fn new(option: &ProxyOptions) -> ProxyResult<Self> {
        let config = Config::from_path(&option.config_path)?;
        // Todo: Add trace error to validate()
        config.validate().unwrap();
        // Todo: Add trace error to validate_product()
        store::validate_product(&config.product_name()).unwrap();

        // Todo: Build ProxyModel with configuration information and system information, replace default.
        // start_time, product_name, data_center, pid, pwd, sys, hostname
        let model = ProxyModel::default();

        let proxy = RawProxy {
            xauth: String::new(),
            model,

            online: false,
            closed: false,

            lproxy_addr: SocketAddr::from_str(&config.proxy_addr()).unwrap(),
            ladmin_addr: SocketAddr::from_str(&config.admin_addr()).unwrap(),

            config: Arc::new(config),
        };

        // Todo: setup

        proxy.start_metrics_json();
        proxy.start_metrics_influxdb();
        proxy.start_metrics_statsd();

        Ok(Self {
            proxy: Arc::new(RwLock::new(proxy)),
        })
    }

    async fn r_lock(&self) -> RwLockReadGuard<'_, RawProxy> {
        self.proxy.read().await
    }

    async fn w_lock(&self) -> RwLockWriteGuard<'_, RawProxy> {
        self.proxy.write().await
    }

    async fn setup(&self, config: &Config) -> ProxyResult<()> {
        // Todo: proto

        // Todo: token

        // Todo: xauth

        // Todo: jodis

        Ok(())
    }

    async fn start(&self) -> ProxyResult<()> {
        if self.is_closed().await {
            return Err(ProxyError::ClosedProxy);
        }
        if self.is_online().await {
            return Ok(());
        }

        self.do_online().await;

        // Todo: start router, jodis

        Ok(())
    }

    async fn close(&self) -> ProxyResult<()> {
        if self.is_closed().await {
            return Ok(());
        }

        self.do_close().await;

        // Todo: close jodis, ladmin, lproxy, router

        Ok(())
    }

    // Todo: serve_admin
    pub async fn serve_admin(&self) {}

    // Todo: serve_proxy
    pub async fn serve_proxy(&self) -> ProxyResult<()> {
        if self.is_closed().await {
            return Err(ProxyError::ClosedProxy);
        }

        // 这里需要一条启动 log
        let proxy = self.proxy.read().await;
        println!("listen will on {:?}", proxy.lproxy_addr);
        // 挂起监听服务
        listen(&proxy.lproxy_addr).await;
        Ok(self.do_close().await)
    }

    // Todo: keep_alive
    async fn keep_alive(&self, _d: Duration) {}

    async fn accept_conn(&self, listener: TcpListener) -> ProxyResult<TcpStream> {
        let mut delay = DelayExp2::new(10, 500, Duration::from_millis(1));

        loop {
            match listener.accept().await {
                Ok((stream, _)) => return Ok(stream),
                Err(err) => {
                    if let Some(e) = err.source() {
                        if let Some(io_err) = e.downcast_ref::<std::io::Error>() {
                            if io_err.kind() == std::io::ErrorKind::WouldBlock {
                                delay.sleep()?;
                                continue;
                            }
                        }
                    }
                    return Err(ProxyError::Listener);
                }
            };
        }
    }
}

impl Proxy {
    async fn get_xauth(&self) -> String {
        let proxy = self.r_lock().await;
        proxy.xauth.to_string()
    }

    // Todo: get model
    async fn get_model(&self) -> ProxyResult<()> {
        Ok(())
    }

    // Todo: get config
    async fn get_config(&self) -> ProxyResult<()> {
        Ok(())
    }

    async fn is_online(&self) -> bool {
        let proxy = self.r_lock().await;
        proxy.online && !proxy.closed
    }

    async fn do_online(&self) {
        let mut proxy = self.w_lock().await;
        proxy.online = true;
    }

    async fn is_closed(&self) -> bool {
        let proxy = self.r_lock().await;
        proxy.closed
    }

    async fn do_close(&self) {
        let mut proxy = self.w_lock().await;
        proxy.closed = true;
    }
}

impl Proxy {
    // Todo: About Slot: has_switched, slots, fill_slot, fill_slots
}

impl Proxy {
    // Todo: About Failover: switch_masters, get_sentinels
}

//实际的挂起函数
async fn listen(addr: &SocketAddr) {
    let listener = TcpListener::bind(addr).await.unwrap();
    while let Ok((stream, _addr)) = listener.accept().await {
        tokio::spawn(do_task(stream));
    }
}

// 简单的打印服务器
async fn do_task(stream: TcpStream) {
    let mut buf_stream = BufStream::new(stream);
    let mut msg = vec![0; 1024];
    loop {
        match buf_stream.read(&mut msg).await {
            Ok(n) if n == 0 => continue,
            Ok(n) => {
                println!("{:?}", String::from_utf8((&msg[..n]).to_vec()));
                let size = buf_stream.write("+OK".as_bytes()).await.unwrap();
                buf_stream.flush().await.unwrap();
                println!("write_size: {}", size);
            }
            Err(e) => break,
        }
    }
}

// Todo: Overview

// Todo: Stats

mod tests {}
