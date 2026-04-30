use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct CreatePost {
    pub title: Option<String>,
    pub content: Option<String>,
}

#[derive(Serialize)]
#[derive(Clone)]
pub struct Post {
    pub id: u64,
    pub title: String,
    pub content: String,
}

#[derive(Serialize)]
pub struct PostSummary {
    pub id: u64,
    pub title: String,
    pub summary: String,
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

#[derive(Serialize)]
pub struct Session {
    pub token: String,
}