use axum::{
    Json, Router, extract::{State}, routing::{post}
};

use crate::{AppState, error::ApiError, model::{AuthRequest, Session}};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
}

pub async fn register(
    State(state): State<AppState>,
    Json(payload): Json<AuthRequest>,
) -> Result<Json<Session>, ApiError> {
    if search_username(&payload.username, &state).await.is_ok() {
        return Err(ApiError::BadRequest("Username already exists".to_string()));
    }
    let hashed_password = hash_password(&payload.password);
    let row = sqlx::query!(
        "INSERT INTO users (username, password_hash) VALUES ($1, $2) RETURNING id",
        payload.username,
        hashed_password
    )
    .fetch_one(&state.pool)
    .await?;

    let session_token = create_session(row.id, &state).await?;
    Ok(Json(Session { token: session_token }))
}

pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<AuthRequest>,
) -> Result<Json<Session>, ApiError> {
    let row = sqlx::query!(
        "SELECT id, password_hash FROM users WHERE username = $1",
        payload.username
    )
    .fetch_one(&state.pool)
    .await
    .map_err(|_| ApiError::BadRequest("Username or password is incorrect".to_string()))?;

    if bcrypt::verify(&payload.password, &row.password_hash).unwrap_or(false) {
        let session_token = create_session(row.id, &state).await?;
        Ok(Json(Session { token: session_token }))
    } else {
        Err(ApiError::BadRequest("Username or password is incorrect".to_string()))
    }
}

fn hash_password(password: &str) -> String {
    bcrypt::hash(password, bcrypt::DEFAULT_COST).unwrap_or_else(|_| format!("hashed_{}", password))
}

async fn create_session(user_id: i32, state: &AppState) -> Result<String, sqlx::Error> {
    // limit one session per user to prevent session spamming

    let row = sqlx::query!(
        "SELECT token FROM sessions WHERE user_id = $1 AND expires_at > NOW()",
        user_id as i64
    )
    .fetch_optional(&state.pool)
    .await?;

    if let Some(session) = row {
        return Ok(session.token);
    }

    use rand::{distributions::Alphanumeric, Rng};
    let token: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(32)
        .map(char::from)
        .collect();

    sqlx::query!(
        "INSERT INTO sessions (user_id, token, expires_at) VALUES ($1, $2, NOW() + INTERVAL '7 days')",
        user_id as i64,
        token
    )
    .execute(&state.pool)
    .await?;

    Ok(token)
}

async fn search_username(username: &str, state: &AppState) -> Result<i32, sqlx::Error> {
    let row = sqlx::query!("SELECT id FROM users WHERE username = $1", username)
        .fetch_one(&state.pool)
        .await?;
    Ok(row.id)
}