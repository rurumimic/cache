use tokio::net::TcpListener;

use crate::{Error, Result};

pub async fn network_init(address: &str, port: u16) -> Result<TcpListener> {
    TcpListener::bind(&format!("{}:{}", address, port))
        .await
        .map_err(|e| Error::Network(e))
}
