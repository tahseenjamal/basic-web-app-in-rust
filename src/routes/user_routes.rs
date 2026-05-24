use axum::{
    routing::{get, post},
    Router,
};
use sqlx::SqlitePool;

use crate::models::user::{create_user, get_user};

pub fn user_routes() -> Router<SqlitePool> {
    Router::new()
        .route("/user", post(create_user))
        .route("/user", get(get_user))
}
