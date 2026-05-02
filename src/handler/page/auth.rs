use askama::Template;
use axum::{
    Extension, 
    Form, 
    Router, 
    extract::State, 
    http::header::{
        SET_COOKIE
    }, 
    response::{
        Html, IntoResponse, Redirect, Response
    }, 
    routing::{
        get, 
        post
    }
};
use crate::{
    AppState, 
    db::{
        session::{
            create_session, 
            invalidate_session
        }, 
        user::{
            authenticate_user, 
            create_user
        }
    }, 
    error::PageError, 
    model::{
        AuthRequest, 
        SessionWithUser
    }
};

pub fn public_routes() -> Router<AppState> {
    Router::new()
        .route("/login", get(login_page))
        .route("/login", post(login))
        .route("/register", get(register_page))
        .route("/register", post(register))
}

pub fn protected_routes() -> Router<AppState> {
    Router::new()
        .route("/logout", get(logout))
}

#[derive(Template)]
#[template(path = "login.html")]
struct LoginTemplate {}

#[derive(Template)]
#[template(path = "register.html")]
struct RegisterTemplate {}

async fn login_page() -> Result<Html<String>, PageError> {
    let template = LoginTemplate {};
    Ok(Html(template.render()?))
}

async fn login(
    State(state): State<AppState>,
    Form(payload): Form<AuthRequest>,
) -> Result<Response, PageError> {
    let user = authenticate_user(&payload.username, &payload.password, &state.pool).await?.ok_or(PageError::BadRequest("Username or password is incorrect".to_string()))?;
    let session = create_session(user.id, &state.pool).await?;

    let cookie = format!("session={}; HttpOnly; SameSite=Strict; Path=/", session.token);
    Ok((
        [(SET_COOKIE, cookie)],
        Redirect::to("/"),
    ).into_response())
}

async fn register_page() -> Result<Html<String>, PageError> {
    let template = RegisterTemplate {};
    Ok(Html(template.render()?))
}

async fn register(
    State(state): State<AppState>,
    Form(payload): Form<AuthRequest>,
) -> Result<Response, PageError> {
    let user = create_user(&payload.username, &payload.password, &state.pool).await?.ok_or(PageError::BadRequest("Username already exists".to_string()))?;
    let session = create_session(user.id, &state.pool).await?;

    let cookie = format!("session={}; HttpOnly; SameSite=Strict; Path=/", session.token);
    Ok((
        [(SET_COOKIE, cookie)],
        Redirect::to("/"),
    ).into_response())
}

async fn logout(
    State(state): State<AppState>,
    Extension(session_with_user): Extension<SessionWithUser>,
) -> Result<Html<String>, PageError> {
    invalidate_session(session_with_user.session.id, &state.pool).await?;
    Ok(Html("<p>You have been logged out. <a href=\"/login\">Login again</a></p>".to_string()))
}