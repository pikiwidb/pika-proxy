use dashmap::DashMap;
use tokio::sync::mpsc::{channel, Receiver, Sender};
use tokio_util::sync::CancellationToken;

use crate::error::Result;
use crate::models::IntoConnectionInfo;
use crate::proxy::backend::connection_pool::db_connection::DbConnection;

mod db_connection;

pub struct ConnectionPool {
    pool: DashMap<String, DbConnection>,
}

impl ConnectionPool {
    pub fn add_remote<I: IntoConnectionInfo>(&mut self, addr: I) -> Result<()> {
        let (tx, mut rx) = channel(1024);
        let pool_cancel_token = CancellationToken::new();
        tokio::spawn(async move {
            // before impl a error channel to process error, use unwrap() here
            let client = DbConnection::new(addr, rx).unwrap();
            let result = client.run(pool_cancel_token).await;
            match result {
                Ok(_) => {}
                Err(_) => {}
            }
        });
        //self.pool.insert(addr.get_id(), tx.clone());
        Ok(())
    }
}
