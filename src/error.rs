use thiserror::Error as ThisError;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(ThisError, Debug)]
pub enum Error {
    #[error("Configuration")]
    Configuration(#[from] Box<dyn std::error::Error + Send + Sync>),

    #[error("Network error")]
    Network(#[from] std::io::Error),

    #[error("Unknown error")]
    Unknown,
}

