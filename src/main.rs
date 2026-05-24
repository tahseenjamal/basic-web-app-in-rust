use crate::routes::routes::create_routes;
use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use std::str::FromStr;

mod models {
    pub mod blog;
    pub mod user;
}
mod routes {
    pub mod blog_routes;
    pub mod routes;
    pub mod user_routes;
}

#[tokio::main]
async fn main() {
    let opts = SqliteConnectOptions::from_str("sqlite://data.db")
        .expect("invalid SQLite URL")
        .create_if_missing(true);

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(opts)
        .await
        .expect("failed to open SQLite database");

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS users (
            username TEXT PRIMARY KEY,
            name     TEXT NOT NULL,
            created  TEXT NOT NULL
        )",
    )
    .execute(&pool)
    .await
    .expect("failed to create users table");

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS blogs (
            id           INTEGER PRIMARY KEY AUTOINCREMENT,
            username     TEXT NOT NULL,
            name         TEXT NOT NULL,
            user_created TEXT NOT NULL,
            tweet        TEXT NOT NULL,
            timestamp    TEXT NOT NULL
        )",
    )
    .execute(&pool)
    .await
    .expect("failed to create blogs table");

    let address = "127.0.0.1:3000";
    let listener = tokio::net::TcpListener::bind(address)
        .await
        .expect("failed to bind — is port 3000 already in use?");
    println!("Server listening on http://{address}");
    axum::serve(listener, create_routes(pool))
        .await
        .expect("server error");
}
