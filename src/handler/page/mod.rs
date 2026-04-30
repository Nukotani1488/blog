use axum::Router;
use crate::AppState;

pub mod root;
pub mod auth;

pub fn public_routes() -> Router<AppState> {
    Router::new()
        .merge(root::public_routes())
        .nest("/auth", auth::public_routes())
}