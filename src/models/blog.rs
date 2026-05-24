use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, SqlitePool};
use time::OffsetDateTime;

use super::user::User;

#[derive(Deserialize)]
pub struct CreateBlogRequest {
    pub username: String,
    pub name: String,
    pub tweet: String,
}

#[derive(Serialize)]
pub struct Blog {
    pub user: User,
    pub tweet: String,
    #[serde(with = "time::serde::rfc3339")]
    pub timestamp: OffsetDateTime,
}

// Flat row as stored in SQLite; converted to Blog for API responses.
#[derive(FromRow)]
struct BlogRow {
    username:     String,
    name:         String,
    user_created: OffsetDateTime,
    tweet:        String,
    timestamp:    OffsetDateTime,
}

impl From<BlogRow> for Blog {
    fn from(r: BlogRow) -> Self {
        Blog {
            user: User { username: r.username, name: r.name, created: r.user_created },
            tweet: r.tweet,
            timestamp: r.timestamp,
        }
    }
}

pub async fn create_blog(
    State(pool): State<SqlitePool>,
    Json(payload): Json<CreateBlogRequest>,
) -> impl IntoResponse {
    let now = OffsetDateTime::now_utc();
    let result = sqlx::query(
        "INSERT INTO blogs (username, name, user_created, tweet, timestamp)
         VALUES (?, ?, ?, ?, ?)",
    )
    .bind(&payload.username)
    .bind(&payload.name)
    .bind(now)
    .bind(&payload.tweet)
    .bind(now)
    .execute(&pool)
    .await;

    match result {
        Ok(_) => {
            let blog = Blog {
                user: User { username: payload.username, name: payload.name, created: now },
                tweet: payload.tweet,
                timestamp: now,
            };
            (StatusCode::CREATED, Json(blog)).into_response()
        }
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

pub async fn get_blogs(State(pool): State<SqlitePool>) -> impl IntoResponse {
    match sqlx::query_as::<_, BlogRow>(
        "SELECT username, name, user_created, tweet, timestamp
         FROM blogs ORDER BY id DESC",
    )
    .fetch_all(&pool)
    .await
    {
        Ok(rows) => Json(rows.into_iter().map(Blog::from).collect::<Vec<_>>()).into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}
