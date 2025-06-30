use std::sync::Arc;
use crate::db::PgPool;
use crate::models::user::{User, NewUser, UpdateUser};
use crate::schema::users::dsl::*;
use crate::utils::error::AppError;
use diesel::prelude::*;
use uuid::Uuid;

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

    pub async fn save_user(&self, new_user: NewUser) -> Result<User, AppError> {
        let mut conn = self.get_conn()?;

        let result = tokio::task::spawn_blocking(move || {
            diesel::insert_into(users)
                .values(&new_user)
                .get_result::<User>(&mut conn)
        })
        .await
        .map_err(AppError::from)??;

        Ok(result)
    }

    pub async fn find_user_by_id(&self, input_id: Uuid) -> Result<Option<User>, AppError> {
        let mut conn = self.get_conn()?;

        let result = tokio::task::spawn_blocking(move || {
            users
                .filter(id.eq(&input_id))
                .first::<User>(&mut conn)
                .optional()
        })
        .await
        .map_err(AppError::from)??;

        Ok(result)
    }

    pub async fn find_user_by_email(&self, input_email: String) -> Result<Option<User>, AppError> {
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

    pub async fn update_user(&self, user_id: Uuid, updates: UpdateUser) -> Result<User, AppError> {
        let mut conn = self.get_conn()?;

        let result = tokio::task::spawn_blocking(move || {
            diesel::update(users.filter(id.eq(user_id)))
                .set(&updates)
                .get_result::<User>(&mut conn)
        })
        .await
        .map_err(AppError::from)??;

        Ok(result)
    }

    pub async fn delete_user(&self, user_id: Uuid) -> Result<(), AppError> {
        let mut conn = self.get_conn()?;

        tokio::task::spawn_blocking(move || {
            diesel::delete(users.filter(id.eq(user_id)))
                .execute(&mut conn)
        })
        .await
        .map_err(AppError::from)??;

        Ok(())
    }
}
