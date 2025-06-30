use axum::Json;
use std::sync::Arc;
// use uuid::Uuid;

use crate::services::auth_service::AuthService;
// use crate::models::user::User;
use crate::dto::{auth::{LoginRequest, SignUpRequest, AuthResponse}, response::ApiResponse};
use crate::utils::error::AppError;

pub struct AuthHandler {
    service: Arc<AuthService>,
}

impl AuthHandler {
    pub fn new(service: Arc<AuthService>) -> Self {
        Self { service }
    }

    pub async fn login(
        &self,
        payload: LoginRequest,
    ) -> Result<Json<ApiResponse<AuthResponse>>, AppError> { // (StatusCode, Json<ApiResponse<()>>)> { :::
                                                  // AppError handles this
        let AuthResponse {user, access_token} = self
            .service
            .login(Json(payload))
            .await?;

        Ok(Json(ApiResponse::success(
            AuthResponse {
                user,
                access_token,
            },
            "User logged in successfully",
        )))
    }

    pub async fn signup(
        &self,
        payload: SignUpRequest,
    ) -> Result<Json<ApiResponse<AuthResponse>>, AppError> {
        let AuthResponse{ user, access_token } = self
            .service
            .signup(Json(payload))
            .await?;

        Ok(Json(ApiResponse::success(
            AuthResponse {
                user,
                access_token,
            },
            "User signed up successfully",
        )))
    }
}
