use axum::{response::IntoResponse, Json};

use super::user::User;

#[derive(serde::Serialize)]
pub struct Blog {
    user: User,
    tweet: String,
    timestamp: u32,
}

impl Blog {
    pub fn new(user: User, tweet: String, timestamp: u32) -> Blog {
        Blog {
            user: user,
            tweet: tweet,
            timestamp: timestamp,
        }
    }
}
pub async fn create_blog() -> impl IntoResponse {
    Json(Blog::new(
        User::new("tahseen".to_string(), "Tahseen".to_string(), 3434),
        "Hello World!".to_string(),
        3434,
    ))
}
pub async fn get_blog() -> impl IntoResponse {
    Json(Blog {
        user: User {
            username: "tahseen".to_string(),
            name: "Tahseen".to_string(),
            created: 3434,
        },
        tweet: "Hello World!".to_string(),
        timestamp: 3434,
    })
}
