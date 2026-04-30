use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Deserialize)]
pub struct CreatePost {
    pub title: Option<String>,
    pub content: Option<String>,
    pub summary: Option<String>,
}

#[derive(Serialize, Clone)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub author: String,
    pub created_at: DateTime<Utc>,
    pub content: String,
}

#[derive(Serialize)]
pub struct PostSummary {
    pub id: i32,
    pub title: String,
    pub author: String,
    pub created_at: DateTime<Utc>,
    pub summary: String,
}

#[derive(Deserialize)]
pub struct PostQuery {
    pub query: Option<String>,
    pub page: Option<u32>,
    pub limit: Option<u32>,
    pub offset: Option<u32>,
}

impl Default for PostQuery {
    fn default() -> Self {
        PostQuery {
            query: None,
            page: Some(1),
            limit: Some(5),
            offset: Some(0),
        }
    }
}

fn empty_string_if_none<S>(value: &Option<String>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    match value {
        Some(s) => serializer.serialize_str(s),
        None => serializer.serialize_str(""),
    }
}

impl Post {
    pub fn summarize(&self) -> PostSummary {
        let summary = if self.content.len() > 100 {
            format!("{}...", &self.content[..100])
        } else {
            self.content.clone()
        };

        PostSummary {
            id: self.id,
            title: self.title.clone(),
            author: self.author.clone(),
            created_at: self.created_at,
            summary,
        }
    }
}

impl From<Post> for PostSummary {
    fn from(post: Post) -> Self {
        post.summarize()
    }
}

#[derive(Deserialize)]
pub struct AuthRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Clone)]
pub struct User {
    pub id: i32,
    pub username: String,
}

#[derive(Serialize, Clone)]
pub struct Session {
    pub id: i32,
    pub token: String,
    pub expires_at: DateTime<Utc>,
    pub user_id: i32,
}

#[derive(Serialize, Clone)]
pub struct SessionWithUser {
    pub session: Session,
    pub user: User,
}