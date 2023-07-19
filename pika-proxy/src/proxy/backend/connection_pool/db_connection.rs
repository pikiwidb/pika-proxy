use crate::error::Result;
use crate::models::{IntoConnectionInfo, Request};
use tokio::sync::mpsc::Receiver;
use tokio_util::sync::CancellationToken;

pub struct DbConnection {
    //client: Client,
    cmd_channel: Receiver<Request>,
}

impl DbConnection {
    pub fn new<I: IntoConnectionInfo>(
        info: I,
        mut request_chan: Receiver<Request>,
    ) -> Result<Self> {
        unimplemented!();
    }

    pub async fn run(&self, cancel: CancellationToken) -> Result<()> {
        // let mut con = self.client.get_async_connection().await?;
        // while !cancel.is_cancelled() {
        //     tokio::select! {
        //         Some(c) =self.cmd_channel.recv() => {
        //             let res = con.req_packed_command(&c.redis_cmd).await?;
        //             let response = Response{};
        //             //c.response_channel.send(response).await?;
        //             println!("res: {:?}", res);
        //         }
        //         _ = cancel.cancelled() => {
        //             break;
        //         }
        //     }
        // }
        Ok(())
    }
}
