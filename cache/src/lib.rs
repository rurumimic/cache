pub mod error;

use tokio::net::TcpListener;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub type Result<T> = std::result::Result<T, error::Error>;

pub const DEFAULT_PORT: u16 = 11211;

pub fn init_tracing() -> Result<()> {
    let tracing_layer = tracing_subscriber::fmt::layer()
        .compact()
        .with_file(true)
        .with_line_number(true)
        .with_thread_ids(true)
        .with_target(false)
        .with_level(true)
        .with_thread_names(false);

    let init = tracing_subscriber::registry()
        .with(tracing_layer)
        .try_init();

    match init {
        Ok(_) => Ok(()),
        Err(_) => Err(error::Error::Configuration()),
    }
}

pub async fn init_network(address: &str, port: u16) -> Result<TcpListener> {
    TcpListener::bind(&format!("{}:{}", address, port)).await.map_err(|_| error::Error::Network())
}

