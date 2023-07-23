use crate::error::Result;

pub struct ConnectionInfo {}

pub trait IntoConnectionInfo: Send + Clone + 'static {
    fn into_connection_info(self) -> Result<ConnectionInfo>;
}
