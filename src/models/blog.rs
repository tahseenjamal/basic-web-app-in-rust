use axum::{response::IntoResponse, Json};
use time::{format_description::well_known::Rfc3339, OffsetDateTime};

use super::user::User;

#[derive(serde::Serialize)]
pub struct Blog {
    user: User,
    tweet: String,
    #[serde(with = "time::serde::rfc3339")]
    timestamp: OffsetDateTime,
}

impl Blog {
    pub fn new(user: User, tweet: String, timestamp: OffsetDateTime) -> Blog {
        Blog {
            user: user,
            tweet: tweet,
            timestamp: timestamp,
        }
    }
}
pub async fn create_blog() -> impl IntoResponse {
    Json(Blog::new(
        User::new(
            "tahseen".to_string(),
            "Tahseen".to_string(),
            OffsetDateTime::parse("2024-06-30T15:45:00+05:30", &Rfc3339).unwrap(),
        ),
        "Hello World!".to_string(),
        OffsetDateTime::parse("2024-06-30T15:45:00+05:30", &Rfc3339).unwrap(),
    ))
}
pub async fn get_blog() -> impl IntoResponse {
    Json(Blog {
        user: User {
            username: "tahseen".to_string(),
            name: "Tahseen".to_string(),
            created: OffsetDateTime::parse("2024-06-30T15:45:00+05:30", &Rfc3339).unwrap(),
        },
        tweet: "Hello World!".to_string(),
        timestamp: OffsetDateTime::parse("2024-06-30T15:45:00+05:30", &Rfc3339).unwrap(),
    })
}
