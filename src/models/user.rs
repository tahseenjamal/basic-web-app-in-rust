use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use sqlx::{error::ErrorKind, FromRow, SqlitePool};
use time::OffsetDateTime;

#[derive(Deserialize)]
pub struct CreateUserRequest {
    pub username: String,
    pub name: String,
}

#[derive(Deserialize)]
pub struct GetUserQuery {
    pub username: String,
}

#[derive(Serialize, FromRow)]
pub struct User {
    pub username: String,
    pub name: String,
    #[serde(with = "time::serde::rfc3339")]
    pub created: OffsetDateTime,
}

pub async fn create_user(
    State(pool): State<SqlitePool>,
    Json(payload): Json<CreateUserRequest>,
) -> impl IntoResponse {
    let created = OffsetDateTime::now_utc();
    let result = sqlx::query(
        "INSERT INTO users (username, name, created) VALUES (?, ?, ?)",
    )
    .bind(&payload.username)
    .bind(&payload.name)
    .bind(created)
    .execute(&pool)
    .await;

    match result {
        Ok(_) => (
            StatusCode::CREATED,
            Json(User { username: payload.username, name: payload.name, created }),
        )
            .into_response(),
        Err(sqlx::Error::Database(ref e)) if e.kind() == ErrorKind::UniqueViolation => {
            (StatusCode::CONFLICT, "username already taken").into_response()
        }
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

pub async fn get_user(
    State(pool): State<SqlitePool>,
    Query(params): Query<GetUserQuery>,
) -> impl IntoResponse {
    match sqlx::query_as::<_, User>(
        "SELECT username, name, created FROM users WHERE username = ?",
    )
    .bind(&params.username)
    .fetch_optional(&pool)
    .await
    {
        Ok(Some(user)) => Json(user).into_response(),
        Ok(None) => (StatusCode::NOT_FOUND, "user not found").into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}
