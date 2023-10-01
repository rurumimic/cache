pub mod error;
pub mod server;

use tokio::net::TcpListener;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub type Result<T> = std::result::Result<T, error::Error>;

pub const DEFAULT_HOST: &str = "127.0.0.1";
pub const DEFAULT_PORT: u16 = 11211;

pub fn trace_init() -> Result<()> {
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

pub async fn network_init(address: &str, port: u16) -> Result<TcpListener> {
    TcpListener::bind(&format!("{}:{}", address, port)).await.map_err(|_| error::Error::Network())
}

