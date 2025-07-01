use std::sync::Arc;
use crate::{
    repositories::user_repo::UserRepository,
    models::user::User,
    utils::error::AppError,
};
use uuid::Uuid;
use reqwest::StatusCode;

pub struct UserService {
    repo: Arc<UserRepository>,
}

impl UserService {
    pub fn new(repo: Arc<UserRepository>) -> Self {
        Self { repo }
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
}
