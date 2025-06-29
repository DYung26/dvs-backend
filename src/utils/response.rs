use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};

use crate::dto::response::ApiResponse;

impl<T: serde::Serialize> IntoResponse for ApiResponse<T> {
    fn into_response(self) -> Response {
        let status = StatusCode::from_u16(self.status).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
        (status, Json(self)).into_response()
    }
}

impl<T: serde::Serialize> ApiResponse<T> {
    pub fn success(data: T, message: &str) -> Self {
        Self {
            status: StatusCode::OK.as_u16(),
            message: message.to_string(),
            data: Some(data),
        }
    }

    pub fn error(status: StatusCode, message: &str) -> Self {
        Self {
            status: status.as_u16(),
            message: message.to_string(),
            data: None,
        }
    }
}
