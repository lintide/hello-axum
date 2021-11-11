use std::net::SocketAddr;

use axum::{Router, routing::{get}};
use tokio;

mod admin;

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    
    let app = Router::new()
        .route("/", get(root))
        .nest("/admin", admin::router());

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root() -> &'static str {
    return "Hello axum\n";
}
