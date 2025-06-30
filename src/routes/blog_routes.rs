use axum::{
    routing::{get, post},
    Router,
};

use crate::models::blog::{create_blog, get_blog};

pub fn blog_routes() -> Router {
    Router::new()
        .route("/blog", post(create_blog))
        .route("/blog", get(get_blog))
}
