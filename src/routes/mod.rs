use axum::{Router, routing::get, Json};
// use std::sync::Arc;
use serde_json::json;

// use crate::handlers::{auth_handler::AuthHandler, user_handler::UserHandler};
use crate::state::AppState;

mod auth_routes;
mod user_routes;

use self::{auth_routes::auth_routes, user_routes::user_routes};

pub fn create_router() -> Router<AppState> {
    Router::new()
        .route("/", get(root))
        .nest("/auth", auth_routes())
        .nest("/users", user_routes())
}

async fn root() -> Json<serde_json::Value> { // &'static str {
    // "Hello, Decentralized Voter!"
    Json(json!({
        "status": "ok",
        "message": "DVS - Decentralized Voting API is running",
        "version": "1.0.0",
    }))
}
