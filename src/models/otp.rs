use diesel::{Queryable, Insertable, Selectable};
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use crate::schema::otps;

#[derive(Debug, Serialize, Deserialize, Queryable, Selectable)]
#[diesel(table_name = otps)]
pub struct OTP {
    pub id: Uuid,
    pub user_id: Uuid,
    pub email: String,
    pub otp: String,
}

#[derive(Debug, Deserialize, Insertable)]
#[diesel(table_name = otps)]
pub struct NewOTP {
    pub user_id: Uuid,
    pub email: String,
    pub otp: String,
}
