use std::sync::Arc;
use crate::{
    repositories::user_repo::UserRepository,
    models::user::{User, UpdateUser},
    utils::error::AppError,
    dto::auth::WalletLoginRequest,
    state::NonceStore,
};
use uuid::Uuid;
use reqwest::StatusCode;
use axum::Json;
use ethers::utils::hash_message;
use ethers::types::{Signature,H160};

pub struct UserService {
    repo: Arc<UserRepository>,
    nonce_store: NonceStore,
}

impl UserService {
    pub fn new(repo: Arc<UserRepository>, nonce_store: NonceStore) -> Self {
        Self { repo, nonce_store }
    }

pub async fn get_user(&self, user_id: Uuid) -> Result<User, AppError> {
        let user: User = self
            .repo
            .find_user_by_id(user_id.clone())
            .await
            .map_err(|e| {
                let msg = format!("DB error: {:?}", e);
                AppError::new(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    msg,
                    Some(e.to_string()),
                )
            })?
            .ok_or_else(|| {
                let msg = format!("User with ID {} not found", user_id);
                AppError::not_found(&msg)
            })?;
        Ok(user)
    }

    pub async fn connect_wallet(
        &self,
        user_id: Uuid,
        Json(WalletLoginRequest { address, signature }): Json<WalletLoginRequest>,
    ) -> Result<(), AppError> {
        let user: User = self
            .repo
            .find_user_by_id(user_id.clone())
            .await
            .map_err(|e| {
                let msg = format!("DB error: {:?}", e);
                AppError::new(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    msg,
                    Some(e.to_string()),
                )
            })?
            .ok_or_else(|| {
                let msg = format!("User with ID {} not found", user_id);
                AppError::not_found(&msg)
            })?;

        // Get nonce
        let mut store = self.nonce_store.lock().await;
        let nonce = store.get(&address).ok_or_else(|| {
            AppError::new(StatusCode::UNAUTHORIZED, "Nonce not found or expired", None)
        })?;

        // Rebuild message
        let message = format!("Login to DVS with this one-time code: {}", nonce);

        // Recover address
        let message_hash = hash_message(message); // EIP-191 signing
        let signature = signature.parse::<Signature>()
            .map_err(|_| AppError::new(StatusCode::UNAUTHORIZED, "Invalid signature format", None))?;

        let recovered = signature.recover(message_hash)
            .map_err(|_| AppError::new(StatusCode::UNAUTHORIZED, "Failed to recover address", None))?;

        // Validate address
        let parsed_address = address.parse::<H160>().map_err(|e| {
            AppError::new(StatusCode::BAD_REQUEST, "Invalid hex in address", Some(e.to_string()))
        })?;
        if recovered != parsed_address {
            return Err(AppError::new(StatusCode::UNAUTHORIZED, "Signature does not match address", None));
        }

        let existing_user = self
            .repo
            .find_user_by_address(address.clone())
            .await
            .map_err(|e| {
                let msg = format!("DB error: {:?}", e);
                AppError::new( // testing out explicit AppError
                    StatusCode::INTERNAL_SERVER_ERROR,
                    msg,
                    Some(e.to_string()),
                )
            })?;
        if existing_user.is_some() && user.address.is_none() {
            return Err(
                AppError::new(
                    StatusCode::CONFLICT,
                    "Address is already connected",
                    None,
                )
            );
        }

        self
            .repo
            .update_user(user.id, UpdateUser {
                address: Some(address.clone()),
                ..Default::default()
            })
            .await
            .map_err(|e| {
                let msg = format!("DB error: {:?}", e);
                AppError::new(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    msg,
                    Some(e.to_string()),
                )
            })?;

        store.remove(&address);

        Ok(())
    }
}
