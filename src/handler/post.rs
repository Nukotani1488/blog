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

impl Default for PostQuery {
    fn default() -> Self {
        PostQuery {
            query: None,
            page: Some(1),
            limit: Some(10),
            offset: Some(0),
        }
    }
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
        "INSERT INTO posts (author_id, title, content) VALUES ($1, $2, $3) RETURNING id, created_at",
        auth_user.user_id,
        title,
        content
    )
    .fetch_one(&state.pool)
    .await?;

    let username = sqlx::query!(
        "SELECT username FROM users WHERE id = $1",
        auth_user.user_id
    )
    .fetch_one(&state.pool)
    .await?;

    Ok(Json(Post {
        id: row.id,
        title: title,
        content: content,
        author: username.username,
        created_at: row.created_at,
    }))
}

pub async fn get_post(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<Post>, ApiError> {
    let post = sqlx::query_as!(
        Post,
        "SELECT p.id, p.title, p.content, u.username AS author, p.created_at FROM posts p JOIN users u ON p.author_id = u.id WHERE p.id = $1",
        id
    )
    .fetch_one(&state.pool)
    .await
    .map_err(|_| ApiError::NotFound)?;

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

    let search_pattern = format!("%{}%", search_query);

    let posts = sqlx::query_as!(
        PostSummary,
        r#"
        SELECT 
            p.id, 
            p.title, 
            u.username AS author, 
            p.created_at, 
            LEFT(p.content, 100) AS summary 
        FROM posts p 
        JOIN users u ON p.author_id = u.id 
        WHERE p.title ILIKE $1 OR p.content ILIKE $1 
        ORDER BY p.created_at DESC 
        LIMIT $2 OFFSET $3
        "#,
        search_pattern,
        limit as i64,
        final_offset as i64,
    )
    .fetch_all(&state.pool)
    .await?;

    Ok(Json(posts))
}