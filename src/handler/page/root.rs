use askama::Template;
use axum::{Router, extract::State, response::Html};
use crate::{AppState, error::{PageError}, model::PostSummary};

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    posts: Vec<PostSummary>,
}

pub fn public_routes() -> Router<AppState> {
    Router::new()
        .route("/", axum::routing::get(index))
}

pub async fn index(
    State(state): State<AppState>,
) -> Result<Html<String>, PageError> {
    let posts = crate::db::post::list_posts(Default::default(), &state.pool).await?;
    let template = IndexTemplate { posts };
    Ok(Html(template.render()?))
}