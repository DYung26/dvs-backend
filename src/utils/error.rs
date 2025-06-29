use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
use std::{error::Error, fmt};
use diesel::result::Error as DieselError;
use tokio::task::JoinError;

#[derive(Debug)]
pub struct AppError {
    pub status: StatusCode,
    pub message: String,
    pub internal_error: Option<String>, // For logs
}

impl AppError {
    pub fn new<M: Into<String>>(status: StatusCode, message: M, internal_error: Option<String>) -> Self {
        AppError {
            status,
            message: message.into(),
            internal_error,
        }
    }

    pub fn internal<E: fmt::Display>(e: E) -> Self {
        AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Internal server error",
            Some(e.to_string()),
        )
    }

    pub fn unauthorized() -> Self {
        AppError::new(StatusCode::UNAUTHORIZED, "Unauthorized", None)
    }

    pub fn bad_request<M: Into<String>>(msg: M) -> Self {
        AppError::new(StatusCode::BAD_REQUEST, msg, None)
    }

    pub fn not_found<M: Into<String>>(msg: M) -> Self {
        AppError::new(StatusCode::NOT_FOUND, msg, None)
    }
}

#[derive(Serialize)]
struct ErrorResponse {
    status: u16,
    message: String,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        if let Some(e) = &self.internal_error {
            eprintln!("⚠️  Internal error: {}", e); // ← Log the internal error here
        }

        let body = Json(ErrorResponse {
            status: self.status.as_u16(),
            message: self.message.clone(),
        });

        (self.status, body).into_response()
    }
}

// ✅ Add Display for string formatting
impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} (status: {})", self.message, self.status)
    }
}

// ✅ Add Error so you can use `?` properly
impl Error for AppError {}

impl From<diesel::r2d2::PoolError> for AppError {
    fn from(err: diesel::r2d2::PoolError) -> Self {
        AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            &format!("DB Pool Error: {}", err),
            Some(err.to_string()),
        )
    }
}

impl From<DieselError> for AppError {
    fn from(err: DieselError) -> Self {
        AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            &format!("Database error: {}", err),
            Some(err.to_string()),
        )
    }
}

impl From<JoinError> for AppError {
    fn from(err: JoinError) -> Self {
        AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Join error when executing blocking task",
            Some(err.to_string()),
        )
    }
}
