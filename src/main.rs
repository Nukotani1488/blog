use std::env;

use dotenv::dotenv;

use sqlx::{PgPool, postgres::PgPoolOptions};

use axum::{
    Router,
};

#[derive(Clone)]
pub struct AppState {
    pool: PgPool,
}


pub mod handler;
pub mod model;
pub mod error;
pub mod middleware;

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

    let public_routes = 
        Router::new()
            .nest("/posts", handler::post::public_routes())
            .nest("/auth", handler::auth::routes()
        );

    let protected_routes = 
        Router::new()
            .nest("/posts", handler::post::protected_routes())
            .layer(axum::middleware::from_fn_with_state(
                state.clone(),
                middleware::auth::auth_middleware,
            )
        );

    let router = Router::new()
        .merge(public_routes)
        .merge(protected_routes)
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(env::var("LISTEN_ADDR").unwrap_or_else(|_| "0.0.0.0:3000".into())).await.unwrap();
    axum::serve(listener, router).await.unwrap();
}