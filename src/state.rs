use std::{
    collections::HashMap,
    sync::Arc,
};
use crate::handlers::{auth_handler::AuthHandler, user_handler::UserHandler};
use tokio::sync::Mutex;

pub type NonceStore = Arc<Mutex<HashMap<String, String>>>;

#[derive(Clone)]
pub struct AppState {
    pub auth_handler: Arc<AuthHandler>,
    pub user_handler: Arc<UserHandler>,
}
