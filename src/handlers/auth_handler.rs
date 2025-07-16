use axum::Json;
use std::sync::Arc;

use crate::{
    services::auth_service::AuthService,
    dto::{
        auth::{
            LoginRequest, SignUpRequest, AuthResponse,
            NonceRequest, NonceResponse, WalletLoginRequest,
        },
        response::ApiResponse,
    },
    utils::error::AppError,
};

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

    pub async fn get_nonce(
        &self,
        payload: NonceRequest,
    ) -> Result<Json<ApiResponse<NonceResponse>>, AppError> {
        let NonceResponse { nonce } = self
            .service
            .get_nonce(Json(payload))
            .await?;

        Ok(Json(ApiResponse::success(
            NonceResponse {
                nonce,
            },
            "Nonce generated successfully",
        )))
    }

    pub async fn wallet_login(
        &self,
        payload: WalletLoginRequest,
    ) -> Result<Json<ApiResponse<AuthResponse>>, AppError> {
        let AuthResponse { user, access_token } = self
            .service
            .wallet_login(Json(payload))
            .await?;

        Ok(Json(ApiResponse::success(
            AuthResponse {
                user,
                access_token,
            },
            "User logged in successfully",
        )))
    }
}
