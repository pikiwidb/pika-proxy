use std::sync::Arc;

use tokio::task::JoinHandle;
use tokio::{
    net::{
        tcp::{OwnedReadHalf, OwnedWriteHalf},
        TcpStream,
    },
    sync::mpsc::{Receiver, Sender},
};

use crate::models::Response;

use crate::error::Result;
use crate::{
    error::Error,
    proxy::{config::Config, router::Router},
};

// session 对应的是一个 client 的连接
pub struct ClientSession {
    router: Arc<dyn Router>,
    config: Arc<Config>,
}

pub struct ClientSessionOption {
    pub router: Arc<dyn Router>,
    pub config: Arc<Config>,
}

impl ClientSession {
    pub fn new(option: ClientSessionOption) -> Self {
        Self {
            router: option.router.clone(),
            config: option.config.clone(),
        }
    }

    fn spawn_writer_task(
        &self,
        writer: OwnedWriteHalf,
        mut response_channel: Receiver<Response>,
    ) -> JoinHandle<u64> {
        tokio::spawn(async move {
            let max_id = 0u64;
            while let Some(response) = response_channel.recv().await {
                // let mut writer = writer.clone();
                // let bytes = response.to_bytes();
                // writer.write_all(&bytes).await?;
                // writer.flush().await?;
            }
            max_id
        })
    }

    fn spawn_reader_task(
        &self,
        client_reader: OwnedReadHalf,
        mut response_channel: Sender<Response>,
    ) -> JoinHandle<u64> {
        tokio::spawn(async move {
            let mut max_id = 0u64;
            let mut request_reader = redis::RedisRequestReader::new(client_reader);
            loop {
                match request_reader.read_request().await {
                    Ok(request) => {
                        //max_id = request.get_id();
                        //let mut response = Response::new();
                        //self.router.dispatch(&request, &mut response);
                        // response_channel.send(response).await?;
                    }
                    Err(e) => {
                        // error!("read request error: {:?}", e);
                    }
                }
            }
            max_id
        })
    }

    pub(crate) fn serve_client(&self, conn: TcpStream) -> Result<()> {
        let (client_reader, client_writer) = conn.into_split();
        let (writer_sender, writer_receiver) = tokio::sync::mpsc::channel(1024);
        self.spawn_reader_task(client_reader, writer_sender);
        self.spawn_writer_task(client_writer, writer_receiver);
        Ok(())
    }
}
