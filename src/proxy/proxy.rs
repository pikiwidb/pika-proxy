use tokio::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};

use super::config::Config;
use std::{net::TcpListener, sync::Arc};

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

    async fn r_lock(&self) -> RwLockReadGuard<'_, _Proxy> {
        self.proxy.read().await
    }

    async fn w_lock(&self) -> RwLockWriteGuard<'_, _Proxy> {
        self.proxy.write().await
    }
}
