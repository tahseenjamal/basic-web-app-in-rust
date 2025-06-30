use axum::Router;

use crate::routes::{blog_routes::blog_routes, user_routes::user_routes};

pub fn create_routes() -> Router {
    Router::new().merge(blog_routes()).merge(user_routes())
}
