use axum::{
    routing::{get, post},
    Router,
};

use crate::models::{
    blog::{create_blog, get_blog},
    user::{create_user, get_user},
};

pub fn create_routes() -> Router {
    Router::new()
        .route("/blog", post(create_blog))
        .route("/user", post(create_user))
        .route("/blog", get(get_blog))
        .route("/user", get(get_user))
}
