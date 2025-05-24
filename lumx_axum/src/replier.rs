use axum::{http::StatusCode, response::IntoResponse};
use serde::Serialize;

pub struct Replier;

impl Replier {
    pub fn ok<T>(content: T) -> axum::Json<T>
    where
        T: Serialize,
    {
        axum::Json(content)
    }

    pub fn render<T>(code: StatusCode, content: T) -> axum::response::Response
    where
        T: Serialize,
    {
        (code, axum::Json(content)).into_response()
    }
}
