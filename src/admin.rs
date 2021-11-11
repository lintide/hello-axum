use axum::{Router, routing::get};


pub fn router() -> Router {
    return Router::new()
        .route("/", get(root))
        .route("/role", get(role));
}

async fn root() -> &'static str {
    return "admin/root page";
}

async fn role() -> &'static str {
    return "admin/role page";
}
