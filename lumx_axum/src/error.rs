use axum::{http::StatusCode, response::IntoResponse};
use lumx_core::types::ProgramFailure;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum WebError {
    #[error("something went wrong")]
    ServerError,
}

impl IntoResponse for WebError {
    fn into_response(self) -> axum::response::Response {
        match self {
            WebError::ServerError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Something went wrong"),
            )
                .into_response(),
        }
    }
}

impl From<ProgramFailure> for WebError {
    fn from(value: ProgramFailure) -> Self {
        match value {
            ProgramFailure::Config(_) => WebError::ServerError,
            ProgramFailure::Database(_) => WebError::ServerError,
            ProgramFailure::Serve(_) => WebError::ServerError,
            ProgramFailure::Unknown(_) => WebError::ServerError,
            ProgramFailure::Scheduler(_) => WebError::ServerError,
            ProgramFailure::ComponentNotExist(_) => WebError::ServerError,
        }
    }
}
