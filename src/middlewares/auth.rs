use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
};
use crate::utils::jwt::verify_token;
use crate::dto::auth::TokenPayload;

#[async_trait]
impl<S> FromRequestParts<S> for TokenPayload
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, String);

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let auth_header = parts
            .headers
            .get("Authorization")
            .ok_or((StatusCode::UNAUTHORIZED, "Missing Authorization header".to_string()))?
            .to_str()
            .map_err(|_| (StatusCode::BAD_REQUEST, "Invalid Authorization header".to_string()))?;

        if !auth_header.starts_with("Bearer ") {
            return Err((StatusCode::BAD_REQUEST, "Expected Bearer token".to_string()));
        }

        let token = &auth_header[7..];
        verify_token(token, false)
            .map_err(|_| (StatusCode::UNAUTHORIZED, "Invalid or expired token".to_string()))
    }
}
