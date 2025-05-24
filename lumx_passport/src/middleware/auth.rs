use lumx_axum::axum::extract::Request;
use lumx_axum::axum::http::StatusCode;
use lumx_axum::axum::middleware::Next;
use lumx_axum::axum::response::{IntoResponse, Response};
use lumx_axum::axum::{http, Json};
use lumx_axum::types::failure::FailureReply;
use lumx_core::program::Program;
use passport_core::decoder::DecodeAccessToken;
use passport_jwt::decoder::AccessTokenDecoder;
use std::sync::Arc;

pub struct JwtAuthError {
    message: String,
    status_code: StatusCode,
}

impl IntoResponse for JwtAuthError {
    fn into_response(self) -> Response {
        let reply = FailureReply {
            message: self.message,
            errors: vec![],
        };

        (self.status_code, Json(reply)).into_response()
    }
}

/// jwt authentication middleware
/// use AccessTokenDecoder to decode authorization header.
pub async fn jwt_auth(mut req: Request, next: Next) -> Result<Response, JwtAuthError> {
    let program = match req.extensions().get::<Arc<Program>>() {
        None => Err(JwtAuthError {
            message: "failed to extract program from extensions".into(),
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
        })?,
        Some(program) => program,
    };

    let header_value = req.headers().get(http::header::AUTHORIZATION);

    let auth_header = match header_value {
        None => Err(JwtAuthError {
            message: "Please add the authorization header".into(),
            status_code: StatusCode::FORBIDDEN,
        }),
        Some(header) => header.to_str().map_err(|_| JwtAuthError {
            message: "Empty header is not allowed".into(),
            status_code: StatusCode::FORBIDDEN,
        }),
    }?;

    let mut header = auth_header.split_whitespace();

    let (_, maybe_token) = (header.next(), header.next());

    let token = match maybe_token {
        None => Err(JwtAuthError {
            message: "Please add the JWT token to the header".into(),
            status_code: StatusCode::FORBIDDEN,
        })?,
        Some(token) => token,
    };

    let decoder = match program.get_component::<AccessTokenDecoder>() {
        None => Err(JwtAuthError {
            message: "AccessTokenDecoder not found".into(),
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
        })?,
        Some(decoder) => decoder,
    };

    let claims_principal = match decoder.decode_access_token(token.into()).await {
        Ok(authentication) => authentication,
        Err(err) => Err(JwtAuthError {
            message: err.to_string(),
            status_code: StatusCode::FORBIDDEN,
        })?,
    };

    req.extensions_mut().insert(claims_principal);

    Ok(next.run(req).await)
}
