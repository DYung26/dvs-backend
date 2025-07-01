use axum::{extract::State, Json, Router, routing::get};
use crate::{
    state::AppState,
    dto::{
        response::ApiResponse,
        auth::TokenPayload,
    },
    utils::error::AppError,
    models::user::User,
};

pub fn user_routes() -> Router<AppState> {
    Router::new()
        .route(
            "/:id",
            get(get_user_handler),
        )
}

async fn get_user_handler(
    State(state): State<AppState>,
    claims: TokenPayload,
    // Json(body): Json<SignUpRequest>,
) -> Result<Json<ApiResponse<User>>, AppError> {
    state.user_handler.get_user(claims.user_id).await
}
