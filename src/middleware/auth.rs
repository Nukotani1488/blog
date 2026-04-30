use axum::{
    extract::{Request, State},
    middleware::Next,
    response::Response,
};

use crate::{
    AppState,
    db::{
        session::{
            get_session_by_token,
            is_valid_session
        },
        user::get_user_by_id
    },
    error::ApiError,
    model::SessionWithUser
};

pub async fn auth_middleware(
    State(state): State<AppState>,
    mut req: Request,
    next: Next,
) -> Result<Response, ApiError> {
    let token = extract_token_from_header(&req).or_else(|| extract_token_from_cookie(&req)).ok_or(ApiError::Unauthorized)?;

    if !is_valid_session(&token, &state.pool).await? {
        return Err(ApiError::Unauthorized);
    }

    let session = get_session_by_token(&token, &state.pool).await?.ok_or(ApiError::Unauthorized)?;
    let user = get_user_by_id(session.user_id, &state.pool).await?.ok_or(ApiError::Unauthorized)?;

    req.extensions_mut().insert(SessionWithUser { session, user });
    Ok(next.run(req).await)
}

fn extract_token_from_cookie(req: &Request) -> Option<String> {
    req.headers()
        .get(axum::http::header::COOKIE)
        .and_then(|cookie_header| cookie_header.to_str().ok())
        .and_then(|cookies| {
            cookies
                .split(';')
                .find_map(|cookie| {
                    let cookie = cookie.trim();
                    if cookie.starts_with("session=") {
                        Some(cookie.trim_start_matches("session=").to_string())
                    } else {
                        None
                    }
                })
        })
}

fn extract_token_from_header(req: &Request) -> Option<String> {
    req.headers()
        .get(axum::http::header::AUTHORIZATION)
        .and_then(|auth_header| auth_header.to_str().ok())
        .and_then(|auth_str| {
            if auth_str.starts_with("Bearer ") {
                Some(auth_str.trim_start_matches("Bearer ").to_string())
            } else {
                None
            }
        })
}