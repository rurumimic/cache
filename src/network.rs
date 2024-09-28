use crate::{Error, Result};
use tokio::net::TcpListener;

pub const DEFAULT_HOST: &str = "127.0.0.1";
pub const DEFAULT_PORT: u16 = 11211;

pub async fn network_init(address: &str, port: u16) -> Result<TcpListener> {
    TcpListener::bind(&format!("{}:{}", address, port))
        .await
        .map_err(|_| Error::Network())
}
