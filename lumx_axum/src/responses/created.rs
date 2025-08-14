use axum::{Json, http::StatusCode, response::IntoResponse};
use serde::Serialize;

#[derive(Serialize)]
pub struct Created<T>
where
    T: Serialize,
{
    pub id: T,
}

impl<T> Created<T>
where
    T: Serialize,
{
    pub fn new(id: T) -> Self {
        Self { id }
    }
}

impl<T> IntoResponse for Created<T>
where
    T: Serialize,
{
    fn into_response(self) -> axum::response::Response {
        (StatusCode::CREATED, Json(self)).into_response()
    }
}
