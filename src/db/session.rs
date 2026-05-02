use sqlx::PgPool;
use rand::{distributions::Alphanumeric, Rng};
use crate::model::Session;

async fn get_active_session_for_user(user_id: i32, pool: &PgPool) -> Result<Option<Session>, sqlx::Error> {
    let session = sqlx::query_as!(
        Session,
        "SELECT id, token, expires_at, user_id FROM sessions WHERE user_id = $1 AND expires_at > NOW()",
        user_id
    )
    .fetch_optional(pool)
    .await?;

    Ok(session)
}

pub async fn create_session(user_id: i32,  pool: &PgPool) -> Result<Session, sqlx::Error> {
    // limit to one session per user to prevent session spamming
    if let Some(existing) = get_active_session_for_user(user_id, pool).await? {
        return Ok(existing);
    }

    let mut token: String;

    loop {
        token = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(64)
            .map(char::from)
            .collect();
        if !get_session_by_token(&token, pool).await?.is_some() {
            break;
        }
    }

    let session = sqlx::query_as!(
        Session,
        "INSERT INTO sessions (user_id, token, expires_at) VALUES ($1, $2, NOW() + INTERVAL '7 days') RETURNING id, token, expires_at, user_id",
        user_id,
        token
    )
    .fetch_one(pool)
    .await?;

    Ok(session)
}

pub async fn get_user_id_from_session(token: &str, pool: &PgPool) -> Result<Option<i32>, sqlx::Error> {
    let row = sqlx::query!(
        "SELECT user_id FROM sessions WHERE token = $1 AND expires_at > NOW()",
        token
    )
    .fetch_optional(pool)
    .await?;

    Ok(row.map(|r| r.user_id))
}

pub async fn delete_user_sessions(user_id: i32, pool: &PgPool) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "DELETE FROM sessions WHERE user_id = $1",
        user_id
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn invalidate_session(id: i32, pool: &PgPool) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "DELETE FROM sessions WHERE id = $1",
        id
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn is_valid_session(token: &str, pool: &PgPool) -> Result<bool, sqlx::Error> {
    let row = sqlx::query!(
        "SELECT id FROM sessions WHERE token = $1 AND expires_at > NOW()",
        token
    )
    .fetch_optional(pool)
    .await?;

    Ok(row.is_some())
}

pub async fn get_session_by_token(token: &str, pool: &PgPool) -> Result<Option<Session>, sqlx::Error> {
    let session = sqlx::query_as!(
        Session,
        "SELECT id, token, expires_at, user_id FROM sessions WHERE token = $1 AND expires_at > NOW()",
        token
    )
    .fetch_optional(pool)
    .await?;
    
    Ok(session)
}