use axum::{
    Extension, Json, Router, extract::{Path, Query, State}, routing::{get, post}
};

use crate::{AppState, error::ApiError, model::{CreatePost, Post, PostSummary, PostQuery, SessionWithUser}, db::post};

pub fn public_routes() -> Router<AppState> {
    Router::new()
        .route("/", get(list_posts))
        .route("/{id}", get(get_post))
}

pub fn protected_routes() -> Router<AppState> {
    Router::new()
        .route("/", post(create_post))
}

pub async fn list_posts(
    State(state): State<AppState>,
    Query(query): Query<PostQuery>,
) -> Result<Json<Vec<PostSummary>>, ApiError> {
    let posts = post::list_posts(query, &state.pool).await?;
    Ok(Json(posts))
}

pub async fn get_post(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<Post>, ApiError> {
    let post = post::get_post_by_id(id, &state.pool).await?.ok_or(ApiError::NotFound)?;
    Ok(Json(post))
}

pub async fn create_post(
    State(state): State<AppState>,
    Extension(session_with_user): Extension<SessionWithUser>,
    Json(payload): Json<CreatePost>,
) -> Result<Json<Post>, ApiError> {
    let post = post::create_post(session_with_user.user, payload, &state.pool).await?;
    Ok(Json(post))
}