use axum::Router;
use crate::AppState;

pub mod root;
pub mod auth;
pub mod post;

pub fn public_routes() -> Router<AppState> {
    Router::new()
        .merge(root::public_routes())
        .nest("/auth", auth::public_routes())
        .nest("/posts", post::public_routes())
}