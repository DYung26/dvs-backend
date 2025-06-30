use std::sync::Arc;
use crate::{
    services::email_service::EmailService,
    repositories::{
        user_repo::UserRepository,
        otp_repo::OTPRepository,
    },
    dto::{
        auth::{LoginRequest, SignUpRequest, AuthResponse},
        email::EmailMessage,
    },
    models::{
        user::NewUser,
        otp::NewOTP,
    },
    utils::{
        error::AppError,
        password::{hash_password, verify_password},
        otp::generate_otp,
    }
};
use axum::{http::StatusCode, Json};

pub struct AuthService {
    repo: Arc<UserRepository>,
    otp_repo: Arc<OTPRepository>,
    email_service: Arc<EmailService>,
}

impl AuthService {
    pub fn new(
        repo: Arc<UserRepository>, otp_repo: Arc<OTPRepository>,
        email_service: Arc<EmailService>,
    ) -> Self {
        Self {
            repo,
            otp_repo,
            email_service,
        }
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
                AppError::not_found(&msg)
            })?;

        let is_valid = verify_password(&password, &user.password)
            .map_err(|_| AppError::internal(format!("Invalid email or password")))?;

        if !is_valid {
            return Err(AppError::unauthorized());
        }
        
        let access_token = Some("".to_string());

        Ok(AuthResponse{
            user,
            access_token,
        })
    }

    pub async fn signup(
        &self,
        Json(SignUpRequest { username, email, password }): Json<SignUpRequest>
    ) -> Result<AuthResponse, AppError> {
        let existing_user = self
            .repo
            .find_user_by_email(email.clone())
            .await
            .map_err(|e| {
                AppError::new(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Failed to check if user already exists",
                    Some(e.to_string()),
                )
            })?;
        if existing_user.is_some() {
            return Err(
                AppError::new(
                    StatusCode::CONFLICT,
                    "Email is already registered",
                    None,
                )
            );
        }

        let hashed_password = hash_password(&password)
            .map_err(|e| AppError::internal(format!("Failed to hash password: {:?}", e)))?;

        let new_user_data = NewUser {
            username: username.clone(),
            email: email.clone(),
            password: hashed_password,
        };

        let user = self
            .repo
            .save_user(new_user_data)
            .await
            .map_err(|e| {
                AppError::new(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Failed to save user to the database",
                    Some(e.to_string())
                )
            })?;

        let otp = generate_otp();
        let otp_data = NewOTP {
            user_id: user.id,
            email: email.clone(),
            otp: otp.clone(),
        };
        self
            .otp_repo
            .save_otp(otp_data)
            .await
            .map_err(|e| {
                AppError::new(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Failed to save OTP to database",
                    Some(e.to_string()),
                )
            })?;

        let msg = EmailMessage{
            to: email.clone(),
            to_name: username.clone(),
            subject: None,
            plain_text: None,
            html_body: None,
            otp,
        };

        self
            .email_service
            .send_verification_email(msg)
            .await
            .map_err(|e| {
                AppError::new(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Failed to send verification email",
                    Some(e.to_string()),
                )
            })?;

        Ok(AuthResponse{
            user,
            access_token: None,
        })
    }
}
