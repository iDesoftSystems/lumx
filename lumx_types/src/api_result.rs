use thiserror::Error;

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("An internal processing error occurred: {0}")]
    Internal(#[from] Box<dyn std::error::Error + Send + Sync>),
}
