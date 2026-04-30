use sqlx::PgPool;
use bcrypt::{hash, verify, DEFAULT_COST};
use crate::model::User;

fn hash_password(password: &str) -> Result<String, bcrypt::BcryptError> {
    hash(password, DEFAULT_COST)
}

fn verify_password(password: &str, hash: &str) -> bool {
    verify(password, hash).unwrap_or(false)
}

pub async fn authenticate_user(username: &str, password: &str, pool: &PgPool) -> Result<Option<User>, sqlx::Error> {
    let record = sqlx::query!(
        "SELECT id, username, password_hash FROM users WHERE username = $1",
        username
    )
    .fetch_optional(pool)
    .await?;

    if let Some(r) = record {
        if verify_password(password, &r.password_hash) {
            return Ok(Some(User { id: r.id, username: r.username }));
        }
    }

    Ok(None)
}

pub async fn get_username(user_id: i32, pool: &PgPool) -> Result<Option<String>, sqlx::Error> {
    let record = sqlx::query!(
        "SELECT username FROM users WHERE id = $1",
        user_id
    )
    .fetch_optional(pool)
    .await?;

    Ok(record.map(|r| r.username))
}

pub async fn get_user_by_id(user_id: i32, pool: &PgPool) -> Result<Option<User>, sqlx::Error> {
    let user = sqlx::query_as!(
        User,
        "SELECT id, username FROM users WHERE id = $1",
        user_id
    )
    .fetch_optional(pool)
    .await?;

    Ok(user)
}

pub async fn get_user_by_username(username: &str, pool: &PgPool) -> Result<Option<User>, sqlx::Error> {
    let user = sqlx::query_as!(
        User,
        "SELECT id, username FROM users WHERE username = $1",
        username
    )
    .fetch_optional(pool)
    .await?;

    Ok(user)
}

pub async fn create_user(username: &str, password: &str, pool: &PgPool) -> Result<User, sqlx::Error> {
    let password_hash = hash_password(&password).map_err(|_| sqlx::Error::Protocol("failed to hash password".into()))?;
    let user = sqlx::query_as! (
        User,
        "INSERT INTO users (username, password_hash) VALUES ($1, $2) RETURNING id, username",
        username,
        password_hash
    )
    .fetch_one(pool)
    .await?;

    Ok(user)
}

pub async fn delete_user(user_id: i32, pool: &PgPool) -> Result<User, sqlx::Error> {
    let user = sqlx::query_as! (
        User,
        "DELETE FROM users WHERE id = $1 RETURNING id, username",
        user_id
    )
    .fetch_one(pool)
    .await?;

    Ok(user)
}

pub async fn edit_username(user_id: i32, new_username: &str, pool: &PgPool) -> Result<User, sqlx::Error> {
    let user = sqlx::query_as! (
        User,
        "UPDATE users SET username = $1 WHERE id = $2 RETURNING id, username",
        new_username,
        user_id
    )
    .fetch_one(pool)
    .await?;

    Ok(user)
}