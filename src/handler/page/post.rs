use askama::Template;
use axum::{
    Router, 
    extract::{
        Query, 
        State
    }, 
    response::Html, 
    routing::get
};
use crate::{
    AppState, 
    error::PageError, 
    model::{
        PostQuery, 
        PostSummary
    },
    db::post::{
        list_posts,
        get_post_count
    }
};

#[derive(Template)]
#[template(path = "post_index.html")]
struct IndexTemplate {
    posts: Vec<PostSummary>,
    page: u32,
    total_pages: u32,
    limit: u32,
    offset: u32,
    search_query: String
}

pub fn public_routes() -> Router<AppState> {
    Router::new()
        .route("/", get(post_index_page))
}

async fn post_index_page(
    State(state): State<AppState>,
    Query(query): Query<PostQuery>
) -> Result<Html<String>, PageError> {
    let posts = list_posts(query.clone(), &state.pool).await?;

    let page = query.page.unwrap_or(1);
    let limit = query.limit.unwrap_or(10);
    let search_query = query.query.clone().unwrap_or("".to_string());
    let search_pattern = format!("%{}%", search_query);
    let offset = query.offset.unwrap_or(0);
    let total = get_post_count(&search_pattern, &state.pool).await?;
    let total_pages = ((total + limit as u64 - 1) / limit as u64) as u32;

    let template = 
        IndexTemplate {
            posts,
            page,
            search_query,
            offset,
            limit,
            total_pages
        };

    Ok(Html(template.render()?))
}