use crate::model::{Post, PostSummary, CreatePost, PostQuery, User};
use sqlx::PgPool;

pub async fn get_post_by_id(id: i32, pool: &PgPool) -> Result<Option<Post>, sqlx::Error> {
    let post = sqlx::query_as!(
        Post,
        "SELECT p.id, p.title, p.content, u.username AS author, p.created_at FROM posts p JOIN users u ON p.author_id = u.id WHERE p.id = $1",
        id
    )
    .fetch_optional(pool)
    .await?;

    Ok(post)
}

pub async fn create_post(user: User, post: CreatePost, pool: &PgPool) -> Result<Post, sqlx::Error> {

    let title = post.title.unwrap_or_else(|| "Untitled".to_string());
    let content = post.content.unwrap_or_else(|| "".to_string());
    let summary = post.summary.unwrap_or_else(|| content.chars().take(200).collect::<String>());

    let post = sqlx::query_as!(
        Post,
        r#"
        WITH inserted AS (
            INSERT INTO posts (author_id, title, content, summary)
            VALUES ($1, $2, $3, $4)
            RETURNING id, title, content, summary, created_at, author_id
        )
        SELECT inserted.id, inserted.title, inserted.content,
            inserted.created_at, users.username AS author
        FROM inserted
        JOIN users ON inserted.author_id = users.id
        "#,
        user.id,
        title,
        content,
        summary,
    )
    .fetch_one(pool)
    .await?;

    Ok(post)
}

pub async fn get_post_count(search_pattern: &str,pool: &PgPool) -> Result<u64, sqlx::Error> {
    let total: i64 = sqlx::query_scalar!(
    "SELECT COUNT(*) FROM posts WHERE title ILIKE $1 OR content ILIKE $1",
    search_pattern,
    )
    .fetch_one(pool)
    .await?
    .unwrap_or(0);

    Ok(total as u64)
}

pub async fn list_posts(query: PostQuery, pool: &PgPool) -> Result<Vec<PostSummary>, sqlx::Error> {
    let page = query.page.unwrap_or(1);
    let limit = query.limit.unwrap_or(10);
    let offset = query.offset.unwrap_or(0);
    let final_offset = (offset + (page - 1) * limit) as i64;

    let search_query = query.query.unwrap_or_default();

    let search_pattern = format!("%{}%", search_query);

    let posts = sqlx::query_as!(
        PostSummary,
        r#"
        SELECT 
            p.id, 
            p.title, 
            u.username AS author, 
            p.created_at, 
            p.summary
        FROM posts p 
        JOIN users u ON p.author_id = u.id 
        WHERE p.title ILIKE $1 OR p.content ILIKE $1 
        ORDER BY p.created_at DESC 
        LIMIT $2 OFFSET $3
        "#,
        search_pattern,
        limit as i64,
        final_offset as i64,
    )
    .fetch_all(pool)
    .await?;

    Ok(posts)
}