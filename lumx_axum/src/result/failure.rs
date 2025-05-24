use axum::http::StatusCode;
use lumx_domain::api::dtos::field::InvalidField;

use crate::{replier::Replier, types::failure::FailureReply};

#[derive(Debug)]
pub enum ApiFailure {
    NotFound(String),
    Unknown(String),
    Conflict(String),
    InvalidFields(Vec<InvalidField>),
    Unauthorized(String),
    Forbidden(String),
}

impl axum::response::IntoResponse for ApiFailure {
    fn into_response(self) -> axum::response::Response {
        match self {
            ApiFailure::NotFound(msg) => {
                Replier::render(StatusCode::NOT_FOUND, FailureReply::from(msg))
            }
            ApiFailure::Unknown(msg) => {
                Replier::render(StatusCode::INTERNAL_SERVER_ERROR, FailureReply::from(msg))
            }
            ApiFailure::Conflict(msg) => {
                Replier::render(StatusCode::CONFLICT, FailureReply::from(msg))
            }
            ApiFailure::InvalidFields(fields) => {
                Replier::render(StatusCode::BAD_REQUEST, FailureReply::from(fields))
            }
            ApiFailure::Unauthorized(msg) => {
                Replier::render(StatusCode::UNAUTHORIZED, FailureReply::from(msg))
            }
            ApiFailure::Forbidden(msg) => {
                Replier::render(StatusCode::FORBIDDEN, FailureReply::from(msg))
            }
        }
    }
}
