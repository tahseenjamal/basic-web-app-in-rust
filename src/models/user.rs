use axum::{response::IntoResponse, Json};

use serde::Serialize;
use time::{format_description::well_known::Rfc3339, OffsetDateTime};

#[derive(Serialize)]
pub struct User {
    pub username: String,
    pub name: String,
    #[serde(with = "time::serde::rfc3339")]
    pub created: OffsetDateTime,
}

impl User {
    pub fn new(username: String, name: String, created: OffsetDateTime) -> Self {
        Self {
            username: username,
            name: name,
            created: created,
        }
    }
}

pub async fn create_user() -> impl IntoResponse {
    Json(User::new(
        "tahseen".to_string(),
        "Tahseen Jamal".to_string(),
        OffsetDateTime::parse("2024-06-30T15:45:00+05:30", &Rfc3339).unwrap(),
    ))
}

pub async fn get_user() -> impl IntoResponse {
    Json(User {
        username: "tahseen".to_string(),
        name: "Tahseen Jamal".to_string(),
        created: OffsetDateTime::parse("2024-06-30T15:45:00+05:30", &Rfc3339).unwrap(),
    })
}
