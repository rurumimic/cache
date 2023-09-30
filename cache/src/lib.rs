mod error;

use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub type Result<T> = std::result::Result<T, error::Error>;

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

