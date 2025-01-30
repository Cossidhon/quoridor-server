use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    pub userid: Uuid,
    pub role: String,
    pub password: String,
    pub email: String,
}