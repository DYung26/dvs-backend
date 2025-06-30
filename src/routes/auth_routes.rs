use axum::{extract::State, Json, Router, routing::post};
use crate::state::AppState;
use crate::dto::{auth::{LoginRequest, SignUpRequest, AuthResponse}, response::ApiResponse};
use crate::utils::error::AppError;

pub fn auth_routes() -> Router<AppState> {
    Router::new()
        .route(
            "/login",
            post(login_handler),
        )
        .route(
            "/signup",
            post(signup_handler),
        )
}

async fn login_handler(
    State(state): State<AppState>,
    Json(body): Json<LoginRequest>,
) -> Result<Json<ApiResponse<AuthResponse>>, AppError> {
    state.auth_handler.login(body).await
}

async fn signup_handler(
    State(state): State<AppState>,
    Json(body): Json<SignUpRequest>,
) -> Result<Json<ApiResponse<AuthResponse>>, AppError> {
    state.auth_handler.signup(body).await
}
