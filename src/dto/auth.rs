use serde::{Deserialize, Serialize};
use crate::models::user::User;
use uuid::Uuid;

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

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenPayload {
    pub user_id: Uuid,
    pub email: String,

    #[serde(flatten)]
    pub registered: Claims,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Claims {
    // pub sub: String,           // subject (user ID, etc.)
    pub exp: usize,            // expiration timestamp
    pub iat: usize,            // issued-at timestamp
}

