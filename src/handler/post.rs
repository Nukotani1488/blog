use axum::{
    Extension, Json, Router, extract::{Path, Query, State}, routing::{get, post}
};
use serde::{Deserialize};
use crate::{AppState, error::ApiError, middleware::auth::AuthUser, model::{CreatePost, Post, PostSummary}};

#[derive(Deserialize)]
pub struct PostQuery {
    query: Option<String>,
    page: Option<u32>,
    limit: Option<u32>,
    offset: Option<u32>,
}

pub fn public_routes() -> Router<AppState> {
    Router::new()
        .route("/", get(list_posts))
        .route("/{id}", get(get_post))
}

pub fn protected_routes() -> Router<AppState> {
    Router::new()
        .route("/", post(create_post))
}

pub async fn create_post(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthUser>,
    Json(payload): Json<CreatePost>,
) -> Result<Json<Post>, ApiError> {
    let title = payload.title.clone().unwrap_or_else(|| "Untitled".to_string());
    let content = payload.content.clone().unwrap_or_else(|| "".to_string());

    let row = sqlx::query!(
        "INSERT INTO posts (title, content, user_id) VALUES ($1, $2, $3) RETURNING id, title, content",
        title,
        content,
        auth_user.user_id
    )
    .fetch_one(&state.pool)
    .await?;

    Ok(Json(Post {
        id: row.id as u64,
        title: row.title,
        content: row.content,
    }))
}

pub async fn get_post(
    State(state): State<AppState>,
    Path(id): Path<u64>,
) -> Result<Json<Post>, ApiError> {
    let row = sqlx::query!("SELECT id, title, content FROM posts WHERE id = $1", id as i64)
        .fetch_one(&state.pool)
        .await
        .map_err(|_| ApiError::NotFound)?;

    let post = Post {
        id: row.id as u64,
        title: row.title,
        content: row.content,
    };

    Ok(Json(post))
}

pub async fn list_posts(
    Query(query): Query<PostQuery>,
    State(state): State<AppState>,
) -> Result<Json<Vec<PostSummary>>, ApiError> {
    let page = query.page.unwrap_or(1);
    let limit = query.limit.unwrap_or(10);
    let offset = query.offset.unwrap_or(0);
    let final_offset = (offset + (page - 1) * limit) as i64;

    let search_query = query.query.unwrap_or_default();

    let rows = 
        sqlx::query!(
            "SELECT id, title, LEFT(content, 100) AS summary FROM posts WHERE title ILIKE $3 OR content ILIKE $3 LIMIT $1 OFFSET $2", limit as i64, final_offset, format!("%{}%", search_query)
        )
        .fetch_all(&state.pool)
        .await?;

    let titles = rows.into_iter().map(|row| PostSummary {
        id: row.id as u64,
        title: row.title,
        summary: row.summary.unwrap_or_else(|| String::from("")),
    }).collect();

    Ok(Json(titles))
}