mod error;

use opentelemetry::global;
use tracing_subscriber::prelude::*;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt};
use tracing_opentelemetry::OpenTelemetryLayer;

pub type Result<T> = std::result::Result<T, error::Error>;

pub fn init_tracing() -> Result<()> {
    let console_layer = console_subscriber::ConsoleLayer::builder()
        .with_default_env()
        .spawn();

    let tracing_layer = tracing_subscriber::fmt::layer()
        .compact()
        .with_file(true)
        .with_line_number(true)
        .with_thread_ids(true)
        .with_target(false)
        .with_level(true)
        .with_thread_names(false);

    global::set_text_map_propagator(opentelemetry_jaeger::Propagator::new());
    let tracer = opentelemetry_jaeger::new_agent_pipeline()
        .with_service_name("cache")
        .install_simple().unwrap();

    let telemetry_layer = tracing_opentelemetry::layer().with_tracer(tracer);

    let init = tracing_subscriber::registry()
        .with(console_layer)
        .with(tracing_layer)
        .with(telemetry_layer)
        .try_init();

    match init {
        Ok(_) => Ok(()),
        Err(_) => Err(error::Error::Configuration()),
    }
}

