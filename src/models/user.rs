use diesel::{Queryable, Insertable, Selectable};
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use crate::schema::users;

#[derive(Debug, Serialize, Deserialize, Queryable, Selectable)]
#[diesel(table_name = users)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,

    #[serde(skip_serializing)]
    pub password: String,
    // pub role: UserRole,
}

#[derive(Debug, Deserialize, Insertable)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub username: String,
    pub email: String,
    pub password: String,
}

/*
 * #[derive(Debug, Serialize, Deserialize)]
 * pub enum UserRole {
 *    Voter,
 *    Candidate,
 *    Committee,
 * }
 */
