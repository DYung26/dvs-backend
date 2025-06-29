use std::sync::Arc;
use crate::services::user_service::UserService;
// use axum::{extract::Path, Json};
// use uuid::Uuid;
// use crate::models::user::User;

pub struct UserHandler {
    service: Arc<UserService>,
}

impl UserHandler {
    pub fn new(service: Arc<UserService>) -> Self {
        Self { service }
    }

    /*pub async fn get_user(&self, Path(user_id): Path<Uuid>) -> Json<Option<User>> {
        // Json(self.service.login())
    }*/
}
