use axum::{extract::State, Json, Router, routing::{get, post}};
use crate::{
    state::AppState,
    dto::{
        response::ApiResponse,
        auth::{TokenPayload, WalletLoginRequest},
    },
    utils::error::AppError,
    models::user::User,
};

pub fn user_routes() -> Router<AppState> {
    Router::new()
        .route(
            "/",
            get(get_user_handler),
        )
        .route(
            "/wallet/connect",
            post(connect_wallet_handler),
        )
}

async fn get_user_handler(
    State(state): State<AppState>,
    claims: TokenPayload,
    // Json(body): Json<SignUpRequest>,
) -> Result<Json<ApiResponse<User>>, AppError> {
    state.user_handler.get_user(claims.user_id).await
}

async fn connect_wallet_handler(
    State(state): State<AppState>,
    claims: TokenPayload,
    Json(body): Json<WalletLoginRequest>,
) -> Result<Json<ApiResponse<()>>, AppError> {
    state.user_handler.connect_wallet(claims.user_id, body).await
}
