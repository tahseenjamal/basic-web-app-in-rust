use axum::{response::IntoResponse, Json};

#[derive(serde::Serialize)]
pub struct User {
    pub username: String,
    pub name: String,
    pub created: u32,
}

impl User {
    pub fn new(username: String, name: String, created: u32) -> Self {
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
        3434,
    ))
}

pub async fn get_user() -> impl IntoResponse {
    Json(User {
        username: "tahseen".to_string(),
        name: "Tahseen Jamal".to_string(),
        created: 3434,
    })
}
