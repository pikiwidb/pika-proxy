use std::sync::atomic::AtomicU32;
#[derive(Debug, Default)]
pub struct ProxyMetrics {
    pub current_connections: AtomicU32,
}
