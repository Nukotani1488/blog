use axum::{
    Extension, Json, Router, extract::State, routing::post
};

use crate::{AppState, error::ApiError, model::{AuthRequest, Session, SessionWithUser}, db::{session, user}};

pub fn public_routes() -> Router<AppState> {
    Router::new()
        .route("/login", post(login))
        .route("/register", post(register))
}

pub fn protected_routes() -> Router<AppState> {
    Router::new()
        .route("/logout", post(logout))
}

pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<AuthRequest>,
) -> Result<Json<Session>, ApiError> {
    let user = user::authenticate_user(&payload.username, &payload.password, &state.pool).await?.ok_or(ApiError::Unauthorized)?;
    let session = session::create_session(user.id, &state.pool).await?;

    Ok(Json(session))
}

pub async fn logout(
    State(state): State<AppState>,
    Extension(session_with_user): Extension<SessionWithUser>,
) -> Result<(), ApiError> {
    session::invalidate_session(session_with_user.session.id, &state.pool).await?;
    Ok(())
}

pub async fn register(
    State(state): State<AppState>,
    Json(payload): Json<AuthRequest>,
) -> Result<Json<Session>, ApiError> {
    let user = user::create_user(&payload.username, &payload.password, &state.pool).await?.ok_or(ApiError::BadRequest("Username already exists".to_string()))?;
    let session = session::create_session(user.id, &state.pool).await?;

    Ok(Json(session))
}