use axum::Router;
use crate::AppState;

pub mod page;

pub fn public_routes() -> Router<AppState> {
    Router::new()
        .merge(page::public_routes())
}