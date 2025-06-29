// use r2d2::Error as R2D2Error;
use std::sync::Arc;
use crate::db::PgPool;
use crate::models::user::User;
use crate::schema::users::dsl::*;
use crate::utils::error::AppError;
use diesel::prelude::*;

pub struct UserRepository {
    pool: Arc<diesel::r2d2::Pool<diesel::r2d2::ConnectionManager<diesel::PgConnection>>>,
}

impl UserRepository {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }

    fn get_conn(&self) -> Result<diesel::r2d2::PooledConnection<diesel::r2d2::ConnectionManager<diesel::PgConnection>>, diesel::r2d2::PoolError> {
        self.pool.get()
    }

    pub async fn save_user() {
    }

    pub async fn find_user_by_id() {
    }

    pub async fn find_user_by_email(&self, input_email: String) -> Result<Option<User>, AppError>{
        let mut conn = self.get_conn()?;

        let result = tokio::task::spawn_blocking(move || {
            users
                .filter(email.eq(&input_email))
                .first::<User>(&mut conn)
                .optional()
        })
        .await
        .map_err(AppError::from)??;

        Ok(result)
    }
}
