//! Unified error handling for the framework

use thiserror::Error;

#[derive(Error, Debug)]
pub enum FrameworkError {
    #[error("Provider error: {0}")]
    Provider(String),

    #[error("Signer error: {0}")]
    Signer(String),

    #[error("Contract error: {0}")]
    Contract(String),

    #[error("Transaction error: {0}")]
    Transaction(String),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Network error: {0}")]
    Network(String),

    #[error(transparent)]
    Other(#[from] eyre::Report),
}

pub type Result<T> = std::result::Result<T, FrameworkError>;
