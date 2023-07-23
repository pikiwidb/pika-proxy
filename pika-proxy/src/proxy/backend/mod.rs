use std::sync::Arc;

use dashmap::DashMap;

use connection_pool::ConnectionPool;

use crate::proxy::config::Config;

mod connection_pool;

pub struct Backend {
    config: Arc<Config>,
    db_connection_pool: DashMap<String, ConnectionPool>,
}

impl Backend {
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config,
            db_connection_pool: DashMap::new(),
        }
    }
}
