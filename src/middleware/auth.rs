use axum::{
    extract::{Request, State},
    middleware::Next,
    response::Response,
};
use crate::{AppState, error::ApiError};

#[derive(Clone)]
pub struct AuthUser {
    pub user_id: i32,
    pub session_id: i32,
}

pub async fn auth_middleware(
    State(state): State<AppState>,
    mut req: Request,
    next: Next,
) -> Result<Response, ApiError> {
    let token = req
        .headers()
        .get("Authorization")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "))
        .ok_or(ApiError::Unauthorized)?;

    let session = sqlx::query!(
        "SELECT id, user_id FROM sessions WHERE token = $1 AND expires_at > NOW()",
        token,
    )
    .fetch_optional(&state.pool)
    .await
    .map_err(|_| ApiError::Internal)?
    .ok_or(ApiError::Unauthorized)?;

    req.extensions_mut().insert(AuthUser { user_id: session.user_id, session_id: session.id });
    Ok(next.run(req).await)
}