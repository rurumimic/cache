mod error;

pub type Result<T> = std::result::Result<T, error::Error>;

pub fn init_tracing() -> Result<()> {
    let subscriber = tracing_subscriber::fmt()
        .compact()
        .with_file(true)
        .with_line_number(true)
        .with_thread_ids(true)
        .with_target(false)
        .with_level(true)
        .with_thread_names(false)
        .finish();

    match tracing::subscriber::set_global_default(subscriber) {
        Ok(_) => Ok(()),
        Err(_) => Err(error::Error::Configuration()),
    }
}
