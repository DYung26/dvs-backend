use serde::{Deserialize, Serialize};
use crate::models::user::User;

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SignUpRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub user: User,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_token: Option<String>,
}
