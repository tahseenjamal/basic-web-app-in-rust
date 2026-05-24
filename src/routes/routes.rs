use axum::{http::Method, Router};
use sqlx::SqlitePool;
use tower_http::cors::{Any, CorsLayer};

use crate::routes::{blog_routes::blog_routes, user_routes::user_routes};

pub fn create_routes(pool: SqlitePool) -> Router {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST])
        .allow_headers(Any);

    Router::new()
        .merge(blog_routes())
        .merge(user_routes())
        .layer(cors)
        .with_state(pool)
}
