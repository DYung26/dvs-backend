use std::sync::Arc;
use crate::db::PgPool;
use crate::models::otp::{OTP, NewOTP};
use crate::schema::otps::dsl::*;
use crate::utils::error::AppError;
use diesel::prelude::*;
use uuid::Uuid;

pub struct OTPRepository {
    pool: Arc<diesel::r2d2::Pool<diesel::r2d2::ConnectionManager<diesel::PgConnection>>>,
}

impl OTPRepository {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }

fn get_conn(&self) -> Result<diesel::r2d2::PooledConnection<diesel::r2d2::ConnectionManager<diesel::PgConnection>>, diesel::r2d2::PoolError> {
        self.pool.get()
    }

    pub async fn save_otp(&self, new_otp: NewOTP) -> Result<OTP, AppError> {
        let mut conn = self.get_conn()?;

        let result = tokio::task::spawn_blocking(move || {
            diesel::insert_into(otps)
                .values(&new_otp)
                .get_result::<OTP>(&mut conn)
        })
        .await
        .map_err(AppError::from)??;

        Ok(result)
    }

    pub async fn find_otp_by_user_id(&self, input_user_id: Uuid) -> Result<Option<OTP>, AppError> {
        let mut conn = self.get_conn()?;

        let result = tokio::task::spawn_blocking(move || {
            otps
                .filter(user_id.eq(&input_user_id))
                .first::<OTP>(&mut conn)
                .optional()
        })
        .await
        .map_err(AppError::from)??;

        Ok(result)
    }

    pub async fn delete_otp(&self, other_id: Uuid) -> Result<(), AppError> {
        let mut conn = self.get_conn()?;

        tokio::task::spawn_blocking(move || {
            diesel::delete(otps.filter(id.eq(&other_id)))
                .execute(&mut conn)
        })
        .await
        .map_err(AppError::from)??;

        Ok(())
    }
}
