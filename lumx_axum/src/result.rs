pub mod failure;

pub type ApiResult<T> = Result<axum::Json<T>, failure::ApiFailure>;
