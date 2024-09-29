use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::{Error, Result};

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
        Err(e) => Err(Error::Configuration(e.into())),
    }
}
