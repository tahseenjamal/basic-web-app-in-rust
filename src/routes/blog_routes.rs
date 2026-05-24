use axum::{
    routing::{get, post},
    Router,
};
use sqlx::SqlitePool;

use crate::models::blog::{create_blog, get_blogs};

pub fn blog_routes() -> Router<SqlitePool> {
    Router::new()
        .route("/blog", post(create_blog))
        .route("/blog", get(get_blogs))
}
