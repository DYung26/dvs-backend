use std::sync::Arc;
use axum::Json;
use uuid::Uuid;
use crate::{
    models::user::User,
    dto::response::ApiResponse,
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
}
