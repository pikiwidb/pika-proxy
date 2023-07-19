use tokio::sync::mpsc::Sender;

use crate::models::response::Response;

pub struct Command {
    //pub redis_cmd: Cmd,
    pub response_channel: Sender<Response>,
    pub id: usize,
}
