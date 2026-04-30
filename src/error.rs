use axum::{
    response::{IntoResponse, Response},
    http::StatusCode,
    Json,
};
use serde::Serialize;

#[derive(Serialize)]
struct ApiErrorBody {
    error: String,
}

pub enum ApiError {
    NotFound,
    #[allow(dead_code)]
    BadRequest(String),
    Unauthorized,
    Internal,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            ApiError::NotFound => (StatusCode::NOT_FOUND, "not found".to_string()),
            ApiError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
            ApiError::Unauthorized => (StatusCode::UNAUTHORIZED, "unauthorized".to_string()),
            ApiError::Internal => (StatusCode::INTERNAL_SERVER_ERROR, "internal error".to_string()),
        };

        (status, Json(ApiErrorBody { error: message })).into_response()
    }
}

impl From<sqlx::Error> for ApiError {
    fn from(_: sqlx::Error) -> Self {
        ApiError::Internal
    }
}