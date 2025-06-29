use crate::routes::routes::create_routes;

mod models {
    pub mod blog;
    pub mod user;
}
mod routes {
    pub mod routes;
}

#[tokio::main]
async fn main() {
    let routes = create_routes();
    let address = "127.0.0.1:3000";
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();
    axum::serve(listener, routes).await.unwrap();
}
