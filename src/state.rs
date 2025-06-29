use std::sync::Arc;
use crate::handlers::{auth_handler::AuthHandler, user_handler::UserHandler};

#[derive(Clone)]
pub struct AppState {
    pub auth_handler: Arc<AuthHandler>,
    pub user_handler: Arc<UserHandler>,
}
