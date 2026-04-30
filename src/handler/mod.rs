use axum::Router;
use crate::AppState;

pub mod api;
pub mod page;

pub fn public_routes() -> axum::Router<AppState> {
    Router::new()
        .nest("/api", api::public_routes())
        .merge(page::public_routes())
}

pub fn protected_routes() -> axum::Router<AppState> {
    Router::new()
        .nest("/api", api::protected_routes())
}