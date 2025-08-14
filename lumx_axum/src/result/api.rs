use axum::response::IntoResponse;
use thiserror::Error;
use validator::ValidationErrors;

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("An internal processing error occurred: {0}")]
    Internal(#[from] Box<dyn std::error::Error + Send + Sync>),

    #[error(transparent)]
    Validation(#[from] ValidationErrors),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        todo!()
    }
}
