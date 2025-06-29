use std::sync::Arc;
use crate::repositories::user_repo::UserRepository;

pub struct UserService {
    repo: Arc<UserRepository>,
}

impl UserService {
    pub fn new(repo: Arc<UserRepository>) -> Self {
        Self { repo }
    }
}
