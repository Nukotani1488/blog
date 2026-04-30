use crate::AppState;
use axum::Router;

pub mod post;
pub mod auth;

pub fn public_routes() -> Router<AppState> {
    Router::new()
        .nest("/posts", post::public_routes())
        .nest("/auth", auth::public_routes())
}

pub fn protected_routes() -> Router<AppState> {
    Router::new()
        .nest("/posts", post::protected_routes())
        .nest("/auth", auth::protected_routes())
}
