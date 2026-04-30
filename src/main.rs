use std::{collections::HashMap, sync::{Arc, Mutex,}, env};

use dotenv::dotenv;

use sqlx::{PgPool, Pool, database, postgres::PgPoolOptions};

use axum::{
    http::StatusCode,
    routing::{get, post},
    Json, Router,
    extract::{State, Path}
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct CreatePost {
    title: String,
    content: String,
}


#[derive(Serialize)]
#[derive(Clone)]
struct Post {
    id: u64,
    title: String,
    content: String,
}


struct App {
    router: Router,
    state: AppState,
}

#[derive(Clone)]
struct AppState {
    pool: PgPool,
}

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .connect(&database_url)
        .await
        .expect("Failed to connect to the database");

    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    let state = AppState {
        pool,
    };

    // build our application with a route
    let router = Router::new()
        .route("/posts", post(create_post))
        .route("/posts", get(list_titles))
        .route("/posts/{id}", get(get_post))
        .with_state(state);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind(env::var("LISTEN_ADDR").unwrap_or_else(|_| "0.0.0.0:3000".into())).await.unwrap();
    axum::serve(listener, router).await.unwrap();
}

async fn create_post(
    State(state): State<AppState>,
    Json(payload): Json<CreatePost>,
) -> Result<Json<Post>, StatusCode> {

    let row = sqlx::query!(
        "INSERT INTO posts (title, content) VALUES ($1, $2) RETURNING id, title, content",
        payload.title,
        payload.content
    )
    .fetch_one(&state.pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(Post {
        id: row.id as u64,
        title: row.title,
        content: row.content,
    }))
}
async fn get_post(
    State(state): State<AppState>,
    Path(id): Path<u64>,
) -> Result<Json<Post>, StatusCode> {
    let row = sqlx::query!("SELECT id, title, content FROM posts WHERE id = $1", id as i64)
        .fetch_one(&state.pool)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    let post = Post {
        id: row.id as u64,
        title: row.title,
        content: row.content,
    };

    Ok(Json(post))
}

async fn list_titles(
    State(state): State<AppState>
) -> Result<Json<Vec<(u64, String)>>, StatusCode> {
    //let titles = posts.iter().map(|(id, post)| (*id, post.title.clone())).collect();
    let rows = sqlx::query!("SELECT id, title FROM posts")
        .fetch_all(&state.pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let titles = rows.into_iter().map(|row| (row.id as u64, row.title)).collect();

    Ok(Json(titles))
}