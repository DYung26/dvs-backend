use std::sync::Arc;
use crate::repositories::user_repo::UserRepository;
use crate::dto::auth::{LoginRequest, AuthResponse};
use axum::{http::StatusCode, Json};
use crate::utils::password::{/*hash_password,*/verify_password};
use crate::utils::error::AppError;

pub struct AuthService {
    repo: Arc<UserRepository>,
}

impl AuthService {
    pub fn new(repo: Arc<UserRepository>) -> Self {
        Self { repo }
    }

    pub async fn login(
        &self,
        Json(LoginRequest { email, password }): Json<LoginRequest>
    ) -> Result<AuthResponse, AppError> {
    // ) -> Result<Json<ApiResponse<T>>, (StatusCode, Json<ApiResponse<()>>)> {
        let user = self
            .repo
            .find_user_by_email(email.clone())
            .await
            .map_err(|e| {
                let msg = format!("DB error: {:?}", e);
                AppError::new( // testing out explicit AppError
                    StatusCode::INTERNAL_SERVER_ERROR,
                    msg,
                    Some(e.to_string()),
                )
            })?
            .ok_or_else(|| {
                let msg = format!("User with email {} not found", email);
                AppError:: not_found(&msg)
            })?;

        let is_valid = verify_password(&password, &user.password)
            .map_err(|_| AppError::internal(format!("Invalid email or password")))?;

        if !is_valid {
            return Err(AppError::unauthorized());
        }
        
        let access_token = "".to_string();

        Ok(AuthResponse{
            user,
            access_token,
        })
    }

    /*pub async fn signup(
    ) -> AuthResponse {
        // let hashedPassword = hash_password(password).unwrap();
    }*/
}
