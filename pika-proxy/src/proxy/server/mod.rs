pub mod proxy_metrics;

use anyhow::{anyhow, bail};
use std::sync::Arc;

use tokio::net::{TcpListener, TcpStream};
use tracing::info;

use super::config::Config;
use crate::error::{Error, Result};
use crate::proxy::backend::Backend;
use crate::proxy::registry::Registry;
use crate::proxy::router::Router;
use crate::proxy::session::client_session::{ClientSession, ClientSessionOption};
use proxy_metrics::ProxyMetrics;

pub struct ProxyServer {
    router: Arc<dyn Router>,
    config: Arc<Config>,
    backend: Arc<Backend>,
    proxy_metrics: Arc<ProxyMetrics>,
}

pub(crate) struct ProxyOptions {
    pub(crate) config_path: String,
}

impl ProxyServer {
    pub async fn server_client(&mut self, conn: TcpStream) -> Result<()> {
        let option = ClientSessionOption {
            router: self.router.clone(),
            config: self.config.clone(),
        };
        let session = ClientSession::new(option);
        session.serve_client(conn)?;
        unimplemented!()
    }

    pub(crate) fn new(option: &ProxyOptions) -> Result<Self> {
        let config = Arc::new(Config::from_path(&option.config_path)?);
        let registry = Self::initialize_registry(config.clone())?;
        let router = Self::initialize_router(config.clone(), registry.clone())?;
        let backend = Self::initialize_backend(config.clone(), router.clone(), registry)?;
        Ok(ProxyServer {
            router,
            backend,
            config,
            proxy_metrics: Arc::<ProxyMetrics>::default(),
        })
    }

    fn initialize_router(config: Arc<Config>, registry: Arc<Registry>) -> Result<Arc<dyn Router>> {
        unimplemented!()
    }

    fn initialize_backend(
        config: Arc<Config>,
        router: Arc<dyn Router>,
        registry: Arc<Registry>,
    ) -> Result<Arc<Backend>> {
        unimplemented!()
    }

    fn initialize_registry(config: Arc<Config>) -> Result<Arc<Registry>> {
        unimplemented!()
    }

    pub async fn serve_proxy(&mut self) -> Result<()> {
        // 这里需要一条启动 log
        info!("listen will on {:?}", self.config.proxy.addr);
        let listener = TcpListener::bind(&self.config.proxy.addr).await?;
        while let Ok((conn, addr)) = listener.accept().await {
            tracing::debug!("new client connection from {}", addr);
            self.proxy_metrics
                .current_connections
                .fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            self.server_client(conn).await?;
        }
        Ok(())
    }
}

//实际的挂起函数
