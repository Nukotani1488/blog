use std::env;

use axum::Router;
use dotenv::dotenv;
use sqlx::{PgPool, postgres::PgPoolOptions};

#[derive(Clone)]
pub struct AppState {
    pool: PgPool,
}

pub mod db;
pub mod error;
pub mod handler;
pub mod middleware;
pub mod model;

#[tokio::main]
async fn main() {
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

    let state = AppState { pool };

    let admin_username = env::var("ADMIN_USERNAME").unwrap_or_else(|_| "admin".into());
    let admin_password = env::var("ADMIN_PASSWORD").unwrap_or_else(|_| "admin".into());
    let _ = db::user::create_user(&admin_username, &admin_password, &state.pool).await;

    let public_routes = handler::public_routes();
    let protected_routes = handler::protected_routes().layer(
        axum::middleware::from_fn_with_state(state.clone(), middleware::auth::auth_middleware),
    );

    let router = Router::new()
        .merge(public_routes)
        .merge(protected_routes)
        .with_state(state);

    let listen_addr = env::var("LISTEN_ADDR").unwrap_or_else(|_| "0.0.0.0:3000".into());
    let listener = tokio::net::TcpListener::bind(listen_addr).await.unwrap();

    axum::serve(listener, router).await.unwrap();
}