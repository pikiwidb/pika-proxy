use super::config::Config;
use crate::utils::error::{PikaProxyError, Result};
use crossbeam_channel::select;
use std::{net::TcpListener, sync::Arc};
use tokio::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};

#[derive(Debug)]
pub struct Proxy {
    proxy: Arc<RwLock<_Proxy>>,
}

#[derive(Debug)]
struct _Proxy {
    xauth: String,

    online: bool,
    closed: bool,

    config: Box<Config>,

    lproxy: TcpListener,
    ladmin: TcpListener,
}

impl Proxy {
    fn from_config(config: &Config) -> Self {
        let proxy = _Proxy {
            xauth: String::new(),

            online: false,
            closed: false,

            config: todo!(),
            lproxy: todo!(),
            ladmin: todo!(),
        };
        Proxy {
            proxy: Arc::new(RwLock::new(proxy)),
        }
    }

    async fn is_online(&self) -> bool {
        let proxy = self.r_lock().await;
        proxy.online && !proxy.closed
    }

    async fn is_closed(&self) -> bool {
        let proxy = self.r_lock().await;
        proxy.closed
    }

    async fn close(&self) {
        let mut proxy = self.w_lock().await;
        proxy.closed = true;
    }

    async fn r_lock(&self) -> RwLockReadGuard<'_, _Proxy> {
        self.proxy.read().await
    }

    async fn w_lock(&self) -> RwLockWriteGuard<'_, _Proxy> {
        self.proxy.write().await
    }

    async fn serve_admin(&self) {}

    async fn serve_proxy(&self) {
        if self.is_closed().await {
            return;
        }

        // 这里需要一条启动 log
        let proxy = self.proxy.read().await;

        tracing::info!("connect to {:?}", proxy.lproxy.local_addr());

        // select! {}
        self.close().await
    }
}

//实际的挂起函数
async fn listen(listener: TcpListener) -> Result<()> {
    let addr = listener
        .local_addr()
        .map_err(|_| PikaProxyError::NetWorkError("get addr errpr".to_string()))?;
    let _lisenter = TcpListener::bind(addr)
        .map_err(|_| PikaProxyError::NetWorkError(format!("connect to {:?} failed", addr)))?;

    for stream in _lisenter.incoming() {
        // 每一个 tcp stream 表示的就是一个 tcp 的链接流, 针对链接流进行分发处理
        // do something
    }
    Ok(())
}
