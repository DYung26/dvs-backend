use axum::{extract::State, Json, Router, routing::post};
use crate::{
    state::AppState,
    dto::{
        auth::{
            LoginRequest, SignUpRequest, AuthResponse,
            NonceRequest, NonceResponse, WalletLoginRequest,
        },
        response::ApiResponse,
    },
    utils::error::AppError,
};

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
        .route(
            "/nonce",
            post(get_nonce_handler),
        )
        .route(
            "/wallet-login",
            post(wallet_login_handler),
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

async fn get_nonce_handler(
    State(state): State<AppState>,
    Json(body): Json<NonceRequest>,
) -> Result<Json<ApiResponse<NonceResponse>>, AppError> {
    state.auth_handler.get_nonce(body).await
}

async fn wallet_login_handler(
    State(state): State<AppState>,
    Json(body): Json<WalletLoginRequest>,
) -> Result<Json<ApiResponse<AuthResponse>>, AppError> {
    state.auth_handler.wallet_login(body).await
}
