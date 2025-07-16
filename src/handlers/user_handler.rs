use std::sync::Arc;
use axum::Json;
use uuid::Uuid;
use crate::{
    models::user::User,
    dto::{
        response::ApiResponse,
        auth::WalletLoginRequest,
    },
    services::user_service::UserService,
    utils::error::AppError,
};

pub struct UserHandler {
    service: Arc<UserService>,
}

impl UserHandler {
    pub fn new(service: Arc<UserService>) -> Self {
        Self { service }
    }

    pub async fn get_user(&self, user_id: Uuid) -> Result<Json<ApiResponse<User>>, AppError> {
        let user = self.service.get_user(user_id).await?;
        Ok(Json(ApiResponse::success(
            user,
            "User retrieved successfully",
        )))
    }

    pub async fn connect_wallet(
        &self,
        user_id: Uuid,
        payload: WalletLoginRequest
    ) -> Result<Json<ApiResponse<()>>, AppError> {
        self
            .service
            .connect_wallet(user_id, Json(payload))
            .await?;

        Ok(Json(ApiResponse::success(
            (),
            "Wallet connected successfully",
        )))
    }
}
