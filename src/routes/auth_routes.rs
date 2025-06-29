use axum::{extract::State, Json, Router, routing::post};
// use std::sync::Arc;
// use crate::handlers::auth_handler::AuthHandler;
use crate::state::AppState;
use crate::dto::{auth::{LoginRequest, AuthResponse}, response::ApiResponse};
use crate::utils::error::AppError;

pub fn auth_routes() -> Router<AppState> {
    Router::new()
        .route(
            "/login",
            post(login_handler),
        )
        // .route("/signup", post(move |body| handler.signup(body)))
}

async fn login_handler(
    State(state): State<AppState>,
    Json(body): Json<LoginRequest>,
) -> Result<Json<ApiResponse<AuthResponse>>, AppError> {
    state.auth_handler.login(body).await
}
