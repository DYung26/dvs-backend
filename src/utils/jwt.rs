use std::env;
use chrono::{Utc, Duration};
use jsonwebtoken::{encode, decode, EncodingKey, DecodingKey, Header, Validation, TokenData, errors::Error};
use uuid::Uuid;
use crate::dto::auth::{TokenPayload, Claims};

pub fn generate_token(user_id: Uuid, email: String, is_refresh: bool) -> Result<String, Error> {
    let secret = if is_refresh {
        env::var("JWT_REFRESH_SECRET").expect("JWT_REFRESH_SECRET must be set")
    } else {
        env::var("JWT_SECRET").expect("JWT_SECRET must be set")
    };

    let expiry = Utc::now() + Duration::days(7);
    let now = Utc::now();

    let claims = TokenPayload {
        user_id,
        email,
        registered: Claims {
            exp: expiry.timestamp() as usize,
            iat: now.timestamp() as usize,
            ..Default::default()
        },
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
}

pub fn verify_token(token_str: &str, is_refresh: bool) -> Result<TokenPayload, Error> {
    let secret = if is_refresh {
        env::var("JWT_REFRESH_SECRET").expect("JWT_REFRESH_SECRET must be set")
    } else {
        env::var("JWT_SECRET").expect("JWT_SECRET must be set")
    };

    let token_data: TokenData<TokenPayload> = decode(
        token_str,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )?;

    Ok(token_data.claims)
}
