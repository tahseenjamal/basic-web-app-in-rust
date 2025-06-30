use axum::{extract::Query, response::IntoResponse, Json};

use serde::{Deserialize, Serialize};
use time::{format_description::well_known::Rfc3339, OffsetDateTime};

#[derive(Deserialize, Serialize)]
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

pub async fn get_user(Query(params): Query<User>) -> impl IntoResponse {
    // For this println to work, Deserialize is required in derive of struct
    println!("{},{}", params.name, params.username);
    Json(User {
        username: params.username,
        name: params.name,
        created: OffsetDateTime::parse("2024-06-30T15:45:00+05:30", &Rfc3339).unwrap(),
    })
}
