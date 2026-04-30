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

pub enum PageError {
    NotFound,
    Unauthorized,
    Internal,
}

pub enum AppError {
    Api(ApiError),
    Page(PageError),
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

impl From<sqlx::Error> for PageError {
    fn from(_: sqlx::Error) -> Self {
        PageError::Internal
    }
}

impl From<askama::Error> for PageError {
    fn from(_: askama::Error) -> Self {
        PageError::Internal
    }
}

impl IntoResponse for PageError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            PageError::NotFound => (StatusCode::NOT_FOUND, "Page not found".to_string()),
            PageError::Unauthorized => (StatusCode::UNAUTHORIZED, "Unauthorized".to_string()),
            PageError::Internal => (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error".to_string()),
        };

        (status, message).into_response()
    }
}